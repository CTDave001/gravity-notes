export class DebouncedTaskQueue {
  private timeout: ReturnType<typeof setTimeout> | null = null;
  private queue: Promise<void> = Promise.resolve();

  constructor(private readonly delay: number) {}

  schedule(task: () => Promise<void>) {
    this.cancelPending();
    this.timeout = setTimeout(() => {
      this.timeout = null;
      void this.enqueue(task);
    }, this.delay);
  }

  flush(task: () => Promise<void>): Promise<void> {
    this.cancelPending();
    return this.enqueue(task);
  }

  cancelPending() {
    if (this.timeout) {
      clearTimeout(this.timeout);
      this.timeout = null;
    }
  }

  drain(): Promise<void> {
    return this.queue;
  }

  dispose() {
    this.cancelPending();
  }

  private enqueue(task: () => Promise<void>): Promise<void> {
    const result = this.queue.catch(() => undefined).then(task);
    this.queue = result.catch(() => undefined);
    return result;
  }
}
