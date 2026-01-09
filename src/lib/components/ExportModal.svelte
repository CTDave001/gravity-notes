<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { jsPDF } from 'jspdf';

  // Unicode font cache - separate for each variant
  let fontCache: { normal?: string; bold?: string; italic?: string; bolditalic?: string } = {};

  // Convert array buffer to base64
  function arrayBufferToBase64(buffer: ArrayBuffer): string {
    const bytes = new Uint8Array(buffer);
    let binary = '';
    for (let i = 0; i < bytes.length; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  }

  // Load all DejaVu Sans font variants for proper bold/italic support
  async function loadUnicodeFonts(): Promise<typeof fontCache> {
    if (fontCache.normal) {
      console.log('[Font] Using cached fonts');
      return fontCache;
    }

    const fontUrls = {
      normal: 'https://cdn.jsdelivr.net/npm/dejavu-fonts-ttf@2.37.3/ttf/DejaVuSans.ttf',
      bold: 'https://cdn.jsdelivr.net/npm/dejavu-fonts-ttf@2.37.3/ttf/DejaVuSans-Bold.ttf',
      italic: 'https://cdn.jsdelivr.net/npm/dejavu-fonts-ttf@2.37.3/ttf/DejaVuSans-Oblique.ttf',
      bolditalic: 'https://cdn.jsdelivr.net/npm/dejavu-fonts-ttf@2.37.3/ttf/DejaVuSans-BoldOblique.ttf',
    };

    try {
      console.log('[Font] Fetching DejaVu Sans font family...');

      // Fetch all variants in parallel
      const [normalRes, boldRes, italicRes, boldItalicRes] = await Promise.all([
        fetch(fontUrls.normal),
        fetch(fontUrls.bold),
        fetch(fontUrls.italic),
        fetch(fontUrls.bolditalic),
      ]);

      if (!normalRes.ok || !boldRes.ok || !italicRes.ok || !boldItalicRes.ok) {
        throw new Error('Failed to fetch one or more font variants');
      }

      const [normalBuf, boldBuf, italicBuf, boldItalicBuf] = await Promise.all([
        normalRes.arrayBuffer(),
        boldRes.arrayBuffer(),
        italicRes.arrayBuffer(),
        boldItalicRes.arrayBuffer(),
      ]);

      fontCache = {
        normal: arrayBufferToBase64(normalBuf),
        bold: arrayBufferToBase64(boldBuf),
        italic: arrayBufferToBase64(italicBuf),
        bolditalic: arrayBufferToBase64(boldItalicBuf),
      };

      console.log('[Font] All font variants loaded successfully');
      return fontCache;
    } catch (err) {
      console.error('[Font] Failed to load fonts:', err);
      return {};
    }
  }

  // Svelte 5 props using $props() rune
  let {
    show = false,
    noteTitle = 'Untitled',
    noteContent = '',
    onclose,
    onsuccess,
  }: {
    show?: boolean;
    noteTitle?: string;
    noteContent?: string;
    onclose?: () => void;
    onsuccess?: (path: string) => void;
  } = $props();

  // PDF Template definitions
  type PdfTemplate = 'clean' | 'professional' | 'academic' | 'modern' | 'compact' | 'custom';

  interface TemplateSettings {
    name: string;
    description: string;
    fontSize: number;
    lineHeight: number;
    marginTop: number;
    marginBottom: number;
    marginSide: number;
    titleSize: number;
    showPageNumbers: boolean;
    showHeader: boolean;
    showDate: boolean;
    headerText?: string;
    codeTheme: 'dark' | 'light';
    accentColor: [number, number, number];
  }

  const templates: Record<PdfTemplate, TemplateSettings> = {
    clean: {
      name: 'Clean',
      description: 'Simple and minimal',
      fontSize: 11,
      lineHeight: 6,
      marginTop: 20,
      marginBottom: 20,
      marginSide: 20,
      titleSize: 22,
      showPageNumbers: false,
      showHeader: false,
      showDate: false,
      codeTheme: 'dark',
      accentColor: [97, 175, 239],
    },
    professional: {
      name: 'Professional',
      description: 'Formal reports & documents',
      fontSize: 11,
      lineHeight: 6.5,
      marginTop: 25,
      marginBottom: 25,
      marginSide: 25,
      titleSize: 24,
      showPageNumbers: true,
      showHeader: true,
      showDate: true,
      codeTheme: 'light',
      accentColor: [50, 80, 140],
    },
    academic: {
      name: 'Academic',
      description: 'Essays & assignments',
      fontSize: 12,
      lineHeight: 8,
      marginTop: 25,
      marginBottom: 25,
      marginSide: 30,
      titleSize: 16,
      showPageNumbers: true,
      showHeader: true,
      showDate: true,
      codeTheme: 'light',
      accentColor: [60, 60, 60],
    },
    modern: {
      name: 'Modern',
      description: 'Contemporary & stylish',
      fontSize: 10.5,
      lineHeight: 6,
      marginTop: 30,
      marginBottom: 30,
      marginSide: 28,
      titleSize: 28,
      showPageNumbers: true,
      showHeader: false,
      showDate: true,
      codeTheme: 'dark',
      accentColor: [139, 92, 246],
    },
    compact: {
      name: 'Compact',
      description: 'Maximum content per page',
      fontSize: 10,
      lineHeight: 5,
      marginTop: 15,
      marginBottom: 15,
      marginSide: 15,
      titleSize: 18,
      showPageNumbers: true,
      showHeader: false,
      showDate: false,
      codeTheme: 'dark',
      accentColor: [80, 80, 80],
    },
    custom: {
      name: 'Custom',
      description: 'Your own settings',
      fontSize: 11,
      lineHeight: 6,
      marginTop: 20,
      marginBottom: 20,
      marginSide: 20,
      titleSize: 22,
      showPageNumbers: false,
      showHeader: false,
      showDate: false,
      codeTheme: 'dark',
      accentColor: [97, 175, 239],
    },
  };

  // Svelte 5 state using $state() rune
  let selectedFormat: 'pdf' | 'md' | 'txt' = $state('md');
  let selectedTemplate: PdfTemplate = $state('clean');
  let showTemplateOptions: boolean = $state(false);
  let customSettings: TemplateSettings = $state({ ...templates.custom });
  let isExporting: boolean = $state(false);
  let error: string | null = $state(null);

  // Get current template settings
  function getCurrentSettings(): TemplateSettings {
    if (selectedTemplate === 'custom') {
      return customSettings;
    }
    return templates[selectedTemplate];
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onclose?.();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onclose?.();
    }
  }

  async function generatePDF(content: string, title: string): Promise<Uint8Array> {
    const settings = getCurrentSettings();

    const doc = new jsPDF({
      orientation: 'portrait',
      unit: 'mm',
      format: 'a4',
    });

    // Try to load Unicode fonts (all variants)
    const fonts = await loadUnicodeFonts();
    let useUnicodeFont = false;

    if (fonts.normal && fonts.bold && fonts.italic && fonts.bolditalic) {
      try {
        console.log('[PDF] Registering font variants...');
        // Register each font variant separately
        doc.addFileToVFS('DejaVuSans.ttf', fonts.normal);
        doc.addFileToVFS('DejaVuSans-Bold.ttf', fonts.bold);
        doc.addFileToVFS('DejaVuSans-Oblique.ttf', fonts.italic);
        doc.addFileToVFS('DejaVuSans-BoldOblique.ttf', fonts.bolditalic);

        doc.addFont('DejaVuSans.ttf', 'DejaVuSans', 'normal');
        doc.addFont('DejaVuSans-Bold.ttf', 'DejaVuSans', 'bold');
        doc.addFont('DejaVuSans-Oblique.ttf', 'DejaVuSans', 'italic');
        doc.addFont('DejaVuSans-BoldOblique.ttf', 'DejaVuSans', 'bolditalic');

        useUnicodeFont = true;
        console.log('[PDF] All font variants registered successfully');
      } catch (err) {
        console.error('[PDF] Failed to register fonts:', err);
      }
    } else {
      console.log('[PDF] Font data incomplete, using fallback');
    }

    const pageWidth = doc.internal.pageSize.getWidth();
    const pageHeight = doc.internal.pageSize.getHeight();
    const margin = settings.marginSide;
    const marginTop = settings.marginTop;
    const marginBottom = settings.marginBottom;
    const maxWidth = pageWidth - margin * 2;
    const lineHeight = settings.lineHeight;
    const codeLineHeight = Math.max(4, settings.lineHeight - 1);
    let y = marginTop;
    let pageNumber = 1;

    // Helper to add page footer (page numbers, header)
    function addPageFooter() {
      if (settings.showPageNumbers) {
        doc.setFont(defaultFont, 'normal');
        doc.setFontSize(9);
        doc.setTextColor(150, 150, 150);
        const pageText = `${pageNumber}`;
        const textWidth = doc.getTextWidth(pageText);
        doc.text(pageText, (pageWidth - textWidth) / 2, pageHeight - 10);
      }
    }

    // Helper to add page header
    function addPageHeader() {
      if (settings.showHeader && pageNumber > 1) {
        doc.setFont(defaultFont, 'normal');
        doc.setFontSize(9);
        doc.setTextColor(150, 150, 150);
        const headerText = settings.headerText || title;
        doc.text(headerText, margin, 12);
        // Header line
        doc.setDrawColor(220, 220, 220);
        doc.setLineWidth(0.3);
        doc.line(margin, 14, pageWidth - margin, 14);
      }
    }

    let inCodeBlock = false;
    let codeBlockLines: string[] = [];
    let inTable = false;
    let tableRows: string[][] = [];

    function checkNewPage(extraSpace = 0): boolean {
      if (y + extraSpace > pageHeight - marginBottom) {
        addPageFooter();
        doc.addPage();
        pageNumber++;
        addPageHeader();
        y = settings.showHeader && pageNumber > 1 ? marginTop + 5 : marginTop;
        return true;
      }
      return false;
    }

    // Set default font based on Unicode support
    const defaultFont = useUnicodeFont ? 'DejaVuSans' : 'helvetica';
    doc.setFont(defaultFont, 'normal');

    // Clean text for safe PDF output
    function cleanText(text: string): string {
      if (!text) return '';

      let result = text;

      // Decode HTML entities
      result = result
        .replace(/&amp;/g, '&')
        .replace(/&lt;/g, '<')
        .replace(/&gt;/g, '>')
        .replace(/&quot;/g, '"')
        .replace(/&#39;/g, "'")
        .replace(/&nbsp;/g, ' ');

      // If using Unicode font, keep all characters
      if (useUnicodeFont) {
        return result;
      }

      // Fallback: replace/strip non-Latin1 for standard fonts
      const safeReplacements: Record<string, string> = {
        '\u2018': "'",  // left single quote
        '\u2019': "'",  // right single quote
        '\u201C': '"',  // left double quote
        '\u201D': '"',  // right double quote
        '…': '...',     // ellipsis
        '—': '--',      // em dash
        '–': '-',       // en dash
      };

      for (const [unicode, ascii] of Object.entries(safeReplacements)) {
        result = result.split(unicode).join(ascii);
      }

      // Remove remaining non-Latin1 characters
      result = result.replace(/[^\x00-\xFF]/g, '');

      return result;
    }

    // Measure text width safely
    function measureText(text: string, fontSize: number = 11): number {
      doc.setFontSize(fontSize);
      return doc.getTextWidth(cleanText(text));
    }

    // Simple text output with proper encoding
    function drawText(text: string, x: number, yPos: number) {
      doc.text(cleanText(text), x, yPos);
    }

    // Word wrap text into lines that fit within maxW
    function wrapText(text: string, maxW: number, fontSize: number = 11): string[] {
      doc.setFont(defaultFont, 'normal');
      doc.setFontSize(fontSize);
      const cleanedText = cleanText(text);
      const words = cleanedText.split(/\s+/);
      const lines: string[] = [];
      let currentLine = '';

      for (const word of words) {
        const testLine = currentLine ? currentLine + ' ' + word : word;
        const testWidth = doc.getTextWidth(testLine);
        if (testWidth > maxW && currentLine) {
          lines.push(currentLine);
          currentLine = word;
        } else {
          currentLine = testLine;
        }
      }
      if (currentLine) {
        lines.push(currentLine);
      }
      // If no wrapping happened but text is still too long, force wrap by character
      if (lines.length === 1 && doc.getTextWidth(lines[0]) > maxW) {
        const chars = lines[0].split('');
        lines.length = 0;
        let line = '';
        for (const char of chars) {
          if (doc.getTextWidth(line + char) > maxW && line) {
            lines.push(line);
            line = char;
          } else {
            line += char;
          }
        }
        if (line) lines.push(line);
      }
      return lines.length > 0 ? lines : [''];
    }

    // Syntax highlighting colors based on theme
    const darkSyntaxColors = {
      keyword: [198, 120, 221],    // Purple - keywords
      string: [152, 195, 121],     // Green - strings
      number: [209, 154, 102],     // Orange - numbers
      comment: [127, 132, 142],    // Gray - comments
      function: [97, 175, 239],    // Blue - function names
      operator: [86, 182, 194],    // Cyan - operators
      default: [171, 178, 191],    // Light gray - default
    };

    const lightSyntaxColors = {
      keyword: [152, 78, 163],     // Purple - keywords
      string: [80, 140, 70],       // Green - strings
      number: [174, 100, 50],      // Orange - numbers
      comment: [140, 140, 150],    // Gray - comments
      function: [50, 100, 180],    // Blue - function names
      operator: [50, 130, 140],    // Cyan - operators
      default: [60, 60, 70],       // Dark gray - default
    };

    const syntaxColors = settings.codeTheme === 'dark' ? darkSyntaxColors : lightSyntaxColors;

    // Tokenize code line for syntax highlighting
    function tokenizeCode(line: string): Array<{ text: string; type: keyof typeof syntaxColors }> {
      const tokens: Array<{ text: string; type: keyof typeof syntaxColors }> = [];
      let remaining = line;

      // Keywords for common languages
      const keywords = /^(const|let|var|function|return|if|else|for|while|do|switch|case|break|continue|class|extends|import|export|from|default|async|await|try|catch|finally|throw|new|this|super|static|public|private|protected|interface|type|enum|implements|extends|null|undefined|true|false|void|typeof|instanceof|in|of|def|self|lambda|print|elif|pass|raise|with|as|yield|None|True|False|fn|mut|pub|use|mod|impl|struct|trait|match|loop|move|ref|where|int|str|bool|float|double|char|long|short|byte|final|abstract|synchronized|volatile|transient|native|package|goto|assert|require|include|define|ifdef|ifndef|endif|pragma|using|namespace|template|typename|virtual|override|explicit|friend|inline|register|extern|sizeof|alignof|decltype|noexcept|constexpr|nullptr|auto|SELECT|FROM|WHERE|JOIN|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER|INDEX|TABLE|INTO|VALUES|SET|AND|OR|NOT|NULL|ORDER|BY|GROUP|HAVING|LIMIT|OFFSET|UNION|DISTINCT|AS|ON|LEFT|RIGHT|INNER|OUTER|FULL|CROSS|PRIMARY|KEY|FOREIGN|REFERENCES|CONSTRAINT|CHECK|UNIQUE|DEFAULT|CASCADE|TRUNCATE|EXISTS|BETWEEN|LIKE|IN|IS|COUNT|SUM|AVG|MIN|MAX|COALESCE|CASE|WHEN|THEN|ELSE|END)(?![a-zA-Z0-9_])/;

      while (remaining.length > 0) {
        // Check for single-line comment
        if (remaining.startsWith('//') || remaining.startsWith('#')) {
          tokens.push({ text: remaining, type: 'comment' });
          break;
        }

        // Check for multi-line comment start (just handle the line portion)
        const multiCommentMatch = remaining.match(/^\/\*.*?(\*\/|$)/);
        if (multiCommentMatch) {
          tokens.push({ text: multiCommentMatch[0], type: 'comment' });
          remaining = remaining.slice(multiCommentMatch[0].length);
          continue;
        }

        // Check for strings (double quotes)
        const doubleStringMatch = remaining.match(/^"(?:[^"\\]|\\.)*"/);
        if (doubleStringMatch) {
          tokens.push({ text: doubleStringMatch[0], type: 'string' });
          remaining = remaining.slice(doubleStringMatch[0].length);
          continue;
        }

        // Check for strings (single quotes)
        const singleStringMatch = remaining.match(/^'(?:[^'\\]|\\.)*'/);
        if (singleStringMatch) {
          tokens.push({ text: singleStringMatch[0], type: 'string' });
          remaining = remaining.slice(singleStringMatch[0].length);
          continue;
        }

        // Check for template literals
        const templateMatch = remaining.match(/^`(?:[^`\\]|\\.)*`/);
        if (templateMatch) {
          tokens.push({ text: templateMatch[0], type: 'string' });
          remaining = remaining.slice(templateMatch[0].length);
          continue;
        }

        // Check for numbers
        const numberMatch = remaining.match(/^-?\d+\.?\d*([eE][+-]?\d+)?/);
        if (numberMatch && (tokens.length === 0 || /[\s\(\[\{,;:=<>!&|+\-*/%]$/.test(tokens[tokens.length - 1]?.text || ''))) {
          tokens.push({ text: numberMatch[0], type: 'number' });
          remaining = remaining.slice(numberMatch[0].length);
          continue;
        }

        // Check for keywords
        const keywordMatch = remaining.match(keywords);
        if (keywordMatch) {
          tokens.push({ text: keywordMatch[0], type: 'keyword' });
          remaining = remaining.slice(keywordMatch[0].length);
          continue;
        }

        // Check for function calls
        const funcMatch = remaining.match(/^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
        if (funcMatch) {
          tokens.push({ text: funcMatch[1], type: 'function' });
          remaining = remaining.slice(funcMatch[1].length);
          continue;
        }

        // Check for operators
        const opMatch = remaining.match(/^(===|!==|==|!=|<=|>=|&&|\|\||=>|->|\+\+|--|<<|>>|\+=|-=|\*=|\/=|[+\-*/%=<>!&|^~?:])/);
        if (opMatch) {
          tokens.push({ text: opMatch[0], type: 'operator' });
          remaining = remaining.slice(opMatch[0].length);
          continue;
        }

        // Default: take one character or a word
        const defaultMatch = remaining.match(/^([a-zA-Z_][a-zA-Z0-9_]*|[^\s]|\s+)/);
        if (defaultMatch) {
          tokens.push({ text: defaultMatch[0], type: 'default' });
          remaining = remaining.slice(defaultMatch[0].length);
        } else {
          tokens.push({ text: remaining[0], type: 'default' });
          remaining = remaining.slice(1);
        }
      }

      return tokens;
    }

    // Render code block with syntax highlighting
    function renderCodeBlock(codeLines: string[], language?: string) {
      if (codeLines.length === 0) return;

      const blockHeight = codeLines.length * codeLineHeight + 6;
      checkNewPage(blockHeight);

      // Draw background for entire block - theme-aware
      if (settings.codeTheme === 'dark') {
        doc.setFillColor(40, 44, 52);
      } else {
        doc.setFillColor(248, 249, 250);
      }
      doc.roundedRect(margin - 2, y - 3, maxWidth + 4, blockHeight, 2, 2, 'F');

      // Draw left border accent
      doc.setFillColor(settings.accentColor[0], settings.accentColor[1], settings.accentColor[2]);
      doc.rect(margin - 2, y - 3, 2, blockHeight, 'F');

      doc.setFont('courier', 'normal');
      doc.setFontSize(9);

      for (const codeLine of codeLines) {
        const displayLine = codeLine.length > 90 ? codeLine.slice(0, 87) + '...' : codeLine;
        const tokens = tokenizeCode(displayLine);

        let x = margin + 2;
        for (const token of tokens) {
          const color = syntaxColors[token.type] || syntaxColors.default;
          doc.setTextColor(color[0], color[1], color[2]);
          doc.text(cleanText(token.text), x, y);
          x += doc.getTextWidth(token.text);
        }

        y += codeLineHeight;
      }

      y += 3;
      doc.setFont(defaultFont, 'normal');
      doc.setFontSize(11);
      doc.setTextColor(50, 50, 50);
    }

    // Render table with adaptive column widths
    function renderTable(rows: string[][]) {
      if (rows.length === 0) return;

      // Filter out separator rows for processing
      const dataRows = rows.filter(row => !row.every(cell => /^[-:]+$/.test(cell)));
      if (dataRows.length === 0) return;

      const colCount = dataRows[0].length;
      doc.setFontSize(9);

      // Calculate column widths based on content
      const colWidths: number[] = [];
      const minColWidth = 20;
      const padding = 4;
      doc.setFont(defaultFont, 'normal');
      doc.setFontSize(9);

      for (let col = 0; col < colCount; col++) {
        let maxContentWidth = minColWidth;
        for (const row of dataRows) {
          if (row[col]) {
            const cellText = cleanText(row[col].trim());
            const cellWidth = doc.getTextWidth(cellText) + padding * 2;
            maxContentWidth = Math.max(maxContentWidth, Math.min(cellWidth, maxWidth / 2)); // Cap at half page width
          }
        }
        colWidths.push(maxContentWidth);
      }

      // Scale widths to fit page if needed
      const totalWidth = colWidths.reduce((a, b) => a + b, 0);
      if (totalWidth > maxWidth) {
        const scale = maxWidth / totalWidth;
        for (let i = 0; i < colWidths.length; i++) {
          colWidths[i] = Math.max(minColWidth, colWidths[i] * scale);
        }
      }

      // Render rows
      for (let rowIdx = 0; rowIdx < dataRows.length; rowIdx++) {
        const row = dataRows[rowIdx];
        const isHeader = rowIdx === 0;

        // Calculate row height based on wrapped content
        let maxRowLines = 1;
        const cellLines: string[][] = [];
        let hasInlineCode = false;

        for (let colIdx = 0; colIdx < row.length; colIdx++) {
          const cellText = (row[colIdx] || '').trim();
          if (cellText.includes('`')) hasInlineCode = true;
          const cellMaxWidth = colWidths[colIdx] - padding * 2;
          const wrapped = wrapText(cellText, cellMaxWidth, 9);
          cellLines.push(wrapped);
          maxRowLines = Math.max(maxRowLines, wrapped.length);
        }

        // Add extra height for inline code (needs more vertical space)
        const baseHeight = maxRowLines * 4.5 + 4;
        const rowHeight = hasInlineCode ? baseHeight + 2 : baseHeight;
        checkNewPage(rowHeight);

        let cellX = margin;
        for (let colIdx = 0; colIdx < row.length; colIdx++) {
          const colWidth = colWidths[colIdx] || minColWidth;

          // Cell background
          if (isHeader) {
            doc.setFillColor(245, 245, 248);
            doc.rect(cellX, y - 3, colWidth, rowHeight, 'F');
            doc.setFont(defaultFont, 'bold');
          } else {
            doc.setFont(defaultFont, 'normal');
          }

          // Cell border
          doc.setDrawColor(220, 220, 220);
          doc.setLineWidth(0.2);
          doc.rect(cellX, y - 3, colWidth, rowHeight, 'S');

          // Cell text (wrapped) with formatting
          doc.setFontSize(9);
          doc.setTextColor(50, 50, 50);
          // Add top padding for text (more if inline code)
          let textY = y + (hasInlineCode ? 1.5 : 0.5);

          for (const cellLine of cellLines[colIdx]) {
            // Render each wrapped line with formatting support
            renderInlineText(cellLine, cellX + padding, textY, 9);
            textY += 4.5;
          }
          cellX += colWidth;
        }

        y += rowHeight;
      }

      doc.setFontSize(11);
      doc.setFont(defaultFont, 'normal');
      y += 4;
    }

    // Render inline formatted text with proper handling
    function renderInlineText(text: string, startX: number, currentY: number, fontSize: number = 11): number {
      let x = startX;
      let remaining = text;

      // Pattern to match inline formatting (order matters - longer patterns first)
      const pattern = /(\*\*\*[^*]+\*\*\*|\*\*[^*]+\*\*|~~[^~]+~~|\*[^*]+\*|`[^`]+`|\[[^\]]+\]\([^)]+\))/;

      while (remaining.length > 0) {
        const match = remaining.match(pattern);

        if (!match || match.index === undefined) {
          // No more formatting, render remaining text
          doc.setFont(defaultFont, 'normal');
          doc.setFontSize(fontSize);
          drawText(remaining, x, currentY);
          x += doc.getTextWidth(remaining);
          break;
        }

        // Render text before match
        if (match.index > 0) {
          const before = remaining.slice(0, match.index);
          doc.setFont(defaultFont, 'normal');
          doc.setFontSize(fontSize);
          drawText(before, x, currentY);
          x += doc.getTextWidth(before);
        }

        const matched = match[0];

        // Inline code
        if (matched.startsWith('`') && matched.endsWith('`')) {
          const codeText = matched.slice(1, -1);

          // Add small gap before code if there's preceding text
          if (x > startX) {
            x += 1;
          }

          doc.setFont('courier', 'normal');
          doc.setFontSize(fontSize - 1);
          const codeWidth = doc.getTextWidth(codeText);

          // Background for inline code with proper padding
          doc.setFillColor(242, 242, 245);
          doc.roundedRect(x, currentY - 3.8, codeWidth + 3, 5.2, 1, 1, 'F');

          doc.setTextColor(180, 70, 70);
          drawText(codeText, x + 1.5, currentY);
          x += codeWidth + 4;

          doc.setTextColor(50, 50, 50);
          doc.setFont(defaultFont, 'normal');
          doc.setFontSize(fontSize);
        }
        // Bold + Italic
        else if (matched.startsWith('***') && matched.endsWith('***')) {
          const content = matched.slice(3, -3);
          doc.setFont(defaultFont, 'bolditalic');
          doc.setFontSize(fontSize);
          drawText(content, x, currentY);
          x += doc.getTextWidth(content);
        }
        // Bold
        else if (matched.startsWith('**') && matched.endsWith('**')) {
          const content = matched.slice(2, -2);
          doc.setFont(defaultFont, 'bold');
          doc.setFontSize(fontSize);
          drawText(content, x, currentY);
          x += doc.getTextWidth(content);
        }
        // Strikethrough
        else if (matched.startsWith('~~') && matched.endsWith('~~')) {
          const content = matched.slice(2, -2);
          doc.setFont(defaultFont, 'normal');
          doc.setFontSize(fontSize);
          doc.setTextColor(130, 130, 130);
          drawText(content, x, currentY);
          const textWidth = doc.getTextWidth(content);
          // Draw strikethrough line
          doc.setDrawColor(130, 130, 130);
          doc.setLineWidth(0.3);
          doc.line(x, currentY - 1.5, x + textWidth, currentY - 1.5);
          x += textWidth;
          doc.setTextColor(50, 50, 50);
        }
        // Italic
        else if (matched.startsWith('*') && matched.endsWith('*')) {
          const content = matched.slice(1, -1);
          doc.setFont(defaultFont, 'italic');
          doc.setFontSize(fontSize);
          drawText(content, x, currentY);
          x += doc.getTextWidth(content);
        }
        // Link
        else if (matched.startsWith('[')) {
          const linkMatch = matched.match(/\[([^\]]+)\]/);
          const linkText = linkMatch ? linkMatch[1] : matched;
          doc.setFont(defaultFont, 'normal');
          doc.setFontSize(fontSize);
          doc.setTextColor(80, 100, 180);
          drawText(linkText, x, currentY);
          x += doc.getTextWidth(linkText);
          doc.setTextColor(50, 50, 50);
        }

        remaining = remaining.slice(match.index + matched.length);
      }

      doc.setFont(defaultFont, 'normal');
      return x;
    }

    // Render paragraph with word wrap and formatting
    function renderParagraph(text: string, startX: number, maxW: number, fontSize: number = 11) {
      doc.setFontSize(fontSize);

      // Strip formatting for width calculation
      const stripFormatting = (t: string) => t
        .replace(/\*\*\*([^*]+)\*\*\*/g, '$1')
        .replace(/\*\*([^*]+)\*\*/g, '$1')
        .replace(/~~([^~]+)~~/g, '$1')
        .replace(/\*([^*]+)\*/g, '$1')
        .replace(/`([^`]+)`/g, '$1')
        .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1');

      const words = text.split(/\s+/);
      let currentLine = '';

      for (const word of words) {
        const testLine = currentLine ? currentLine + ' ' + word : word;
        const cleanTest = stripFormatting(testLine);

        if (doc.getTextWidth(cleanTest) > maxW && currentLine) {
          checkNewPage(lineHeight);
          renderInlineText(currentLine, startX, y, fontSize);
          y += lineHeight;
          currentLine = word;
        } else {
          currentLine = testLine;
        }
      }

      if (currentLine) {
        checkNewPage(lineHeight);
        renderInlineText(currentLine, startX, y, fontSize);
        y += lineHeight;
      }
    }

    // === MAIN RENDERING ===

    // Title
    doc.setFontSize(settings.titleSize);
    doc.setFont(defaultFont, 'bold');
    doc.setTextColor(30, 30, 30);
    drawText(title, margin, y);
    y += settings.titleSize * 0.5 + 2;

    // Date (if enabled)
    if (settings.showDate) {
      doc.setFontSize(10);
      doc.setFont(defaultFont, 'normal');
      doc.setTextColor(120, 120, 120);
      const dateStr = new Date().toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      });
      drawText(dateStr, margin, y);
      y += 6;
    }

    // Divider line
    doc.setDrawColor(settings.accentColor[0], settings.accentColor[1], settings.accentColor[2]);
    doc.setLineWidth(selectedTemplate === 'modern' ? 1 : 0.5);
    doc.line(margin, y, selectedTemplate === 'modern' ? margin + 40 : pageWidth - margin, y);
    y += 10;

    // Reset for content
    doc.setFontSize(settings.fontSize);
    doc.setFont(defaultFont, 'normal');
    doc.setTextColor(50, 50, 50);

    const lines = content.split('\n');

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      // Code block handling
      if (line.trim().startsWith('```')) {
        if (!inCodeBlock) {
          inCodeBlock = true;
          codeBlockLines = [];
        } else {
          inCodeBlock = false;
          renderCodeBlock(codeBlockLines);
          codeBlockLines = [];
        }
        continue;
      }

      if (inCodeBlock) {
        codeBlockLines.push(line);
        continue;
      }

      // Table detection - check for pipe-separated content
      const trimmedForTable = line.trim();
      const pipeCount = (line.match(/\|/g) || []).length;
      const isTableLine = pipeCount >= 2 || (trimmedForTable.startsWith('|') && trimmedForTable.endsWith('|'));

      if (isTableLine && line.includes('|')) {
        // Parse cells - handle both | cell | cell | and cell | cell formats
        let cells: string[];
        if (trimmedForTable.startsWith('|') && trimmedForTable.endsWith('|')) {
          cells = line.split('|').slice(1, -1).map(c => c.trim());
        } else if (trimmedForTable.startsWith('|')) {
          cells = line.split('|').slice(1).map(c => c.trim());
        } else if (trimmedForTable.endsWith('|')) {
          cells = line.split('|').slice(0, -1).map(c => c.trim());
        } else {
          cells = line.split('|').map(c => c.trim());
        }

        if (cells.length > 0) {
          if (!inTable) {
            inTable = true;
            tableRows = [];
          }
          tableRows.push(cells);

          // Check if next line is not a table
          const nextLine = lines[i + 1];
          const nextPipeCount = nextLine ? (nextLine.match(/\|/g) || []).length : 0;
          const nextIsTable = nextLine && (nextPipeCount >= 2 || (nextLine.trim().startsWith('|') && nextLine.trim().endsWith('|')));

          if (!nextIsTable) {
            renderTable(tableRows);
            inTable = false;
            tableRows = [];
          }
          continue;
        }
      }

      checkNewPage(lineHeight);

      // Helper for list detection
      const trimmedLine = line.trimStart();
      const isUnordered = trimmedLine.startsWith('- ') || trimmedLine.startsWith('* ') || trimmedLine.startsWith('+ ');
      const isOrdered = /^\d+\.\s/.test(trimmedLine);

      // Headers
      if (line.startsWith('# ')) {
        y += 4;
        checkNewPage(12);
        doc.setFontSize(18);
        doc.setFont(defaultFont, 'bold');
        doc.setTextColor(30, 30, 30);
        renderInlineText(line.slice(2), margin, y, 18);
        y += 10;
        doc.setFontSize(11);
        doc.setFont(defaultFont, 'normal');
        doc.setTextColor(50, 50, 50);
      } else if (line.startsWith('## ')) {
        y += 3;
        checkNewPage(10);
        doc.setFontSize(15);
        doc.setFont(defaultFont, 'bold');
        doc.setTextColor(40, 40, 40);
        renderInlineText(line.slice(3), margin, y, 15);
        y += 8;
        doc.setFontSize(11);
        doc.setFont(defaultFont, 'normal');
        doc.setTextColor(50, 50, 50);
      } else if (line.startsWith('### ')) {
        y += 2;
        checkNewPage(8);
        doc.setFontSize(13);
        doc.setFont(defaultFont, 'bold');
        doc.setTextColor(50, 50, 50);
        renderInlineText(line.slice(4), margin, y, 13);
        y += 7;
        doc.setFontSize(11);
        doc.setFont(defaultFont, 'normal');
      } else if (line.startsWith('#### ')) {
        y += 1;
        checkNewPage(7);
        doc.setFont(defaultFont, 'bold');
        renderInlineText(line.slice(5), margin, y, 11);
        y += 6;
        doc.setFont(defaultFont, 'normal');
      }
      // Horizontal rule
      else if (line.match(/^(-{3,}|_{3,}|\*{3,})$/)) {
        y += 3;
        doc.setDrawColor(220, 220, 220);
        doc.setLineWidth(0.4);
        doc.line(margin, y, pageWidth - margin, y);
        y += 5;
      }
      // Blockquote
      else if (line.startsWith('>')) {
        const quoteText = line.replace(/^>\s*/, '');

        // Calculate wrapped lines for height
        const wrappedLines = wrapText(quoteText, maxWidth - 12, 11);
        const blockHeight = wrappedLines.length * lineHeight + 4;

        checkNewPage(blockHeight);

        // Background and border
        doc.setFillColor(250, 250, 252);
        doc.roundedRect(margin, y - 3, maxWidth, blockHeight, 1, 1, 'F');
        doc.setFillColor(150, 150, 180);
        doc.rect(margin, y - 3, 2, blockHeight, 'F');

        doc.setTextColor(100, 100, 110);
        doc.setFont(defaultFont, 'italic');
        for (const wrapLine of wrappedLines) {
          renderInlineText(wrapLine, margin + 6, y, 11);
          y += lineHeight;
        }
        doc.setFont(defaultFont, 'normal');
        doc.setTextColor(50, 50, 50);
        y += 2;
      }
      // Unordered list
      else if (isUnordered) {
        const leadingSpaces = line.match(/^(\s*)/)?.[1]?.length || 0;
        const indentLevel = Math.floor(leadingSpaces / 2);
        const bulletX = margin + (indentLevel * 6);
        let textX = bulletX + 5;
        let listText = trimmedLine.replace(/^[-*+]\s+/, '');

        // Check for checkbox syntax: [ ] or [x] or [X]
        const checkboxMatch = listText.match(/^\[([ xX])\]\s*/);
        const isCheckbox = checkboxMatch !== null;
        const isChecked = checkboxMatch && (checkboxMatch[1] === 'x' || checkboxMatch[1] === 'X');

        if (isCheckbox) {
          // Remove checkbox syntax from text
          listText = listText.replace(/^\[[ xX]\]\s*/, '');
          textX = bulletX + 8; // More space for checkbox

          // Draw checkbox
          doc.setDrawColor(100, 100, 100);
          doc.setLineWidth(0.4);
          doc.roundedRect(bulletX, y - 3.5, 4, 4, 0.5, 0.5, 'S');

          if (isChecked) {
            // Draw checkmark
            doc.setDrawColor(60, 140, 80);
            doc.setLineWidth(0.6);
            // Checkmark path
            doc.line(bulletX + 0.8, y - 1.5, bulletX + 1.6, y - 0.7);
            doc.line(bulletX + 1.6, y - 0.7, bulletX + 3.2, y - 2.8);
          }
        } else {
          // Draw bullet point
          doc.setFillColor(80, 80, 80);
          doc.circle(bulletX + 1.5, y - 1.2, 0.8, 'F');
        }

        // Render list text with wrapping
        const listMaxWidth = maxWidth - (indentLevel * 6) - (isCheckbox ? 8 : 5);
        const wrappedLines = wrapText(listText, listMaxWidth, 11);

        for (let li = 0; li < wrappedLines.length; li++) {
          if (li > 0) {
            checkNewPage(lineHeight);
          }
          renderInlineText(wrappedLines[li], li === 0 ? textX : textX, y);
          y += lineHeight;
        }
      }
      // Ordered list
      else if (isOrdered) {
        const leadingSpaces = line.match(/^(\s*)/)?.[1]?.length || 0;
        const indentLevel = Math.floor(leadingSpaces / 2);
        const numMatch = trimmedLine.match(/^(\d+)\./);
        const num = numMatch ? numMatch[1] : '1';
        const numX = margin + (indentLevel * 6);
        const textX = numX + 8;
        const listText = trimmedLine.replace(/^\d+\.\s+/, '');

        // Draw number
        doc.setFont(defaultFont, 'normal');
        doc.setFontSize(11);
        doc.setTextColor(50, 50, 50);
        drawText(num + '.', numX, y);

        // Render list text with wrapping
        const listMaxWidth = maxWidth - (indentLevel * 6) - 8;
        const wrappedLines = wrapText(listText, listMaxWidth, 11);

        for (let li = 0; li < wrappedLines.length; li++) {
          if (li > 0) {
            checkNewPage(lineHeight);
          }
          renderInlineText(wrappedLines[li], li === 0 ? textX : textX, y);
          y += lineHeight;
        }
      }
      // Empty line
      else if (line.trim() === '') {
        y += lineHeight * 0.5;
      }
      // Regular paragraph
      else {
        renderParagraph(line, margin, maxWidth);
      }
    }

    // Add final page footer
    addPageFooter();

    return doc.output('arraybuffer') as unknown as Uint8Array;
  }

  async function handleExport() {
    error = null;
    isExporting = true;

    try {
      const downloadsDir: string = await invoke('get_downloads_dir');

      // Sanitize filename
      const safeFilename = (noteTitle || 'Untitled')
        .replace(/[<>:"/\\|?*]/g, '_')
        .slice(0, 100);

      let outputPath: string;

      if (selectedFormat === 'pdf') {
        // Generate PDF (async to load Unicode font if needed)
        const pdfBytes = await generatePDF(noteContent, noteTitle || 'Untitled');
        outputPath = await invoke('export_pdf', {
          content: Array.from(new Uint8Array(pdfBytes)),
          filename: safeFilename,
          destination: downloadsDir,
        });
      } else {
        // Export as text/markdown
        outputPath = await invoke('export_note_file', {
          content: noteContent,
          filename: safeFilename,
          format: selectedFormat,
          destination: downloadsDir,
        });
      }

      // Notify success with file path
      onsuccess?.(outputPath);
      onclose?.();
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      isExporting = false;
    }
  }

  function selectFormat(format: 'pdf' | 'md' | 'txt') {
    selectedFormat = format;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={handleBackdropClick}
  >
    <div class="bg-white dark:bg-surface-900 rounded-xl shadow-xl w-full max-w-lg mx-4 overflow-hidden max-h-[90vh] flex flex-col">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-surface-200 dark:border-surface-700 shrink-0">
        <h2 class="text-lg font-semibold text-surface-900 dark:text-surface-100">Export Note</h2>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-1">Choose a format to export your note</p>
      </div>

      <!-- Format selection -->
      <div class="px-6 py-4 overflow-y-auto flex-1">
        <div class="grid grid-cols-3 gap-3">
          <!-- PDF button -->
          <button
            class="flex flex-col items-center p-4 rounded-lg border-2 transition-all {selectedFormat === 'pdf'
              ? 'border-accent bg-accent/10 text-accent'
              : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600 text-surface-600 dark:text-surface-400'}"
            onclick={() => selectFormat('pdf')}
          >
            <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 13h6m-6 3h4" />
            </svg>
            <span class="text-sm font-medium">PDF</span>
            <span class="text-xs text-surface-400 dark:text-surface-500 mt-1">.pdf file</span>
          </button>

          <!-- Markdown button -->
          <button
            class="flex flex-col items-center p-4 rounded-lg border-2 transition-all {selectedFormat === 'md'
              ? 'border-accent bg-accent/10 text-accent'
              : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600 text-surface-600 dark:text-surface-400'}"
            onclick={() => selectFormat('md')}
          >
            <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 9h1l1 2 1-2h1M9 15h6" />
            </svg>
            <span class="text-sm font-medium">Markdown</span>
            <span class="text-xs text-surface-400 dark:text-surface-500 mt-1">.md file</span>
          </button>

          <!-- Text button -->
          <button
            class="flex flex-col items-center p-4 rounded-lg border-2 transition-all {selectedFormat === 'txt'
              ? 'border-accent bg-accent/10 text-accent'
              : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600 text-surface-600 dark:text-surface-400'}"
            onclick={() => selectFormat('txt')}
          >
            <svg class="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 9h6M9 12h6M9 15h4" />
            </svg>
            <span class="text-sm font-medium">Plain Text</span>
            <span class="text-xs text-surface-400 dark:text-surface-500 mt-1">.txt file</span>
          </button>
        </div>

        <!-- PDF Template Selection -->
        {#if selectedFormat === 'pdf'}
          <div class="mt-4 pt-4 border-t border-surface-200 dark:border-surface-700">
            <div class="flex items-center justify-between mb-3">
              <span class="text-sm font-medium text-surface-700 dark:text-surface-300">Template</span>
              <button
                class="text-xs text-surface-500 hover:text-surface-700 dark:hover:text-surface-300 transition-colors"
                onclick={() => showTemplateOptions = !showTemplateOptions}
              >
                {showTemplateOptions ? 'Hide options' : 'Show options'}
              </button>
            </div>

            <!-- Template Cards -->
            <div class="grid grid-cols-3 gap-2">
              {#each Object.entries(templates) as [key, template]}
                <button
                  class="p-2.5 rounded-lg border text-left transition-all {selectedTemplate === key
                    ? 'border-accent bg-accent/5 ring-1 ring-accent/30'
                    : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600'}"
                  onclick={() => selectedTemplate = key as PdfTemplate}
                >
                  <div class="text-xs font-medium text-surface-800 dark:text-surface-200">{template.name}</div>
                  <div class="text-[10px] text-surface-500 dark:text-surface-400 mt-0.5 line-clamp-1">{template.description}</div>
                </button>
              {/each}
            </div>

            <!-- Expanded Template Options -->
            {#if showTemplateOptions}
              <div class="mt-4 p-3 rounded-lg bg-surface-50 dark:bg-surface-800/50 space-y-3">
                <!-- Font Size -->
                <div class="flex items-center justify-between">
                  <label class="text-xs text-surface-600 dark:text-surface-400">Font Size</label>
                  <div class="flex items-center gap-2">
                    <input
                      type="range"
                      min="9"
                      max="14"
                      step="0.5"
                      value={getCurrentSettings().fontSize}
                      oninput={(e) => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.fontSize = parseFloat((e.target as HTMLInputElement).value);
                      }}
                      class="w-20 h-1.5 accent-accent"
                    />
                    <span class="text-xs text-surface-500 w-6">{getCurrentSettings().fontSize}</span>
                  </div>
                </div>

                <!-- Line Height -->
                <div class="flex items-center justify-between">
                  <label class="text-xs text-surface-600 dark:text-surface-400">Line Height</label>
                  <div class="flex items-center gap-2">
                    <input
                      type="range"
                      min="4"
                      max="10"
                      step="0.5"
                      value={getCurrentSettings().lineHeight}
                      oninput={(e) => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.lineHeight = parseFloat((e.target as HTMLInputElement).value);
                      }}
                      class="w-20 h-1.5 accent-accent"
                    />
                    <span class="text-xs text-surface-500 w-6">{getCurrentSettings().lineHeight}</span>
                  </div>
                </div>

                <!-- Margins -->
                <div class="flex items-center justify-between">
                  <label class="text-xs text-surface-600 dark:text-surface-400">Margins</label>
                  <div class="flex items-center gap-2">
                    <input
                      type="range"
                      min="10"
                      max="35"
                      step="5"
                      value={getCurrentSettings().marginSide}
                      oninput={(e) => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        const val = parseInt((e.target as HTMLInputElement).value);
                        customSettings.marginSide = val;
                        customSettings.marginTop = val;
                        customSettings.marginBottom = val;
                      }}
                      class="w-20 h-1.5 accent-accent"
                    />
                    <span class="text-xs text-surface-500 w-6">{getCurrentSettings().marginSide}mm</span>
                  </div>
                </div>

                <!-- Code Theme -->
                <div class="flex items-center justify-between">
                  <label class="text-xs text-surface-600 dark:text-surface-400">Code Theme</label>
                  <div class="flex gap-1">
                    <button
                      class="px-2.5 py-1 text-[10px] font-medium rounded transition-colors {getCurrentSettings().codeTheme === 'dark'
                        ? 'bg-surface-800 text-surface-200'
                        : 'bg-surface-200 text-surface-600 dark:bg-surface-700 dark:text-surface-400'}"
                      onclick={() => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.codeTheme = 'dark';
                      }}
                    >
                      Dark
                    </button>
                    <button
                      class="px-2.5 py-1 text-[10px] font-medium rounded transition-colors {getCurrentSettings().codeTheme === 'light'
                        ? 'bg-surface-200 text-surface-800 dark:bg-surface-600 dark:text-surface-100'
                        : 'bg-surface-100 text-surface-500 dark:bg-surface-800 dark:text-surface-500'}"
                      onclick={() => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.codeTheme = 'light';
                      }}
                    >
                      Light
                    </button>
                  </div>
                </div>

                <!-- Toggles row -->
                <div class="flex items-center gap-4 pt-2 border-t border-surface-200 dark:border-surface-700">
                  <label class="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={getCurrentSettings().showPageNumbers}
                      onchange={(e) => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.showPageNumbers = (e.target as HTMLInputElement).checked;
                      }}
                      class="w-3.5 h-3.5 rounded accent-accent"
                    />
                    <span class="text-[10px] text-surface-600 dark:text-surface-400">Page #</span>
                  </label>
                  <label class="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={getCurrentSettings().showHeader}
                      onchange={(e) => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.showHeader = (e.target as HTMLInputElement).checked;
                      }}
                      class="w-3.5 h-3.5 rounded accent-accent"
                    />
                    <span class="text-[10px] text-surface-600 dark:text-surface-400">Header</span>
                  </label>
                  <label class="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={getCurrentSettings().showDate}
                      onchange={(e) => {
                        if (selectedTemplate !== 'custom') {
                          customSettings = { ...templates[selectedTemplate] };
                          selectedTemplate = 'custom';
                        }
                        customSettings.showDate = (e.target as HTMLInputElement).checked;
                      }}
                      class="w-3.5 h-3.5 rounded accent-accent"
                    />
                    <span class="text-[10px] text-surface-600 dark:text-surface-400">Date</span>
                  </label>
                </div>
              </div>
            {/if}
          </div>
        {/if}

        {#if error}
          <div class="mt-4 p-3 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
            {error}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 border-t border-surface-200 dark:border-surface-700 flex justify-end gap-3 shrink-0">
        <button
          class="px-4 py-2 text-sm font-medium text-surface-600 dark:text-surface-400 hover:bg-surface-100 dark:hover:bg-surface-800 rounded-lg transition-colors"
          onclick={onclose}
          disabled={isExporting}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent-hover disabled:opacity-50 disabled:cursor-not-allowed rounded-lg transition-colors"
          onclick={handleExport}
          disabled={isExporting}
        >
          {#if isExporting}
            Exporting...
          {:else}
            Export
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
