import { afterEach, describe, expect, it, vi } from 'vitest';
import { DebouncedTaskQueue } from './autosave';

afterEach(() => {
  vi.useRealTimers();
});

describe('DebouncedTaskQueue', () => {
  it('runs only the latest debounced snapshot', async () => {
    vi.useFakeTimers();
    const queue = new DebouncedTaskQueue(200);
    const saved: string[] = [];

    queue.schedule(async () => { saved.push('old'); });
    queue.schedule(async () => { saved.push('new'); });

    await vi.advanceTimersByTimeAsync(200);
    await queue.drain();

    expect(saved).toEqual(['new']);
  });

  it('flushes immediately and accepts empty content', async () => {
    vi.useFakeTimers();
    const queue = new DebouncedTaskQueue(200);
    const saved: string[] = [];

    queue.schedule(async () => { saved.push('stale'); });
    await queue.flush(async () => { saved.push(''); });

    expect(saved).toEqual(['']);
  });

  it('continues after a failed task', async () => {
    const queue = new DebouncedTaskQueue(0);

    await expect(queue.flush(async () => {
      throw new Error('disk full');
    })).rejects.toThrow('disk full');

    let completed = false;
    await queue.flush(async () => {
      completed = true;
    });

    expect(completed).toBe(true);
  });
});
