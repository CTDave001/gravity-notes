import { invoke } from '@tauri-apps/api/core';

export interface RuntimeInfo {
  platform: string;
  mobile: boolean;
}

let runtimePromise: Promise<RuntimeInfo> | null = null;

export const isIosBuild = import.meta.env.VITE_GRAVITY_PLATFORM === 'ios';

function browserFallback(): RuntimeInfo {
  const userAgent = navigator.userAgent.toLowerCase();
  const forcedMobilePreview = new URLSearchParams(window.location.search).get('mobilePreview') === '1';
  const mobile =
    forcedMobilePreview ||
    /iphone|ipad|ipod|android/.test(userAgent) ||
    (window.matchMedia('(pointer: coarse)').matches && window.innerWidth <= 900);

  return {
    platform: /iphone|ipad|ipod/.test(userAgent) ? 'ios' : 'browser',
    mobile,
  };
}

export function getRuntimeInfo(): Promise<RuntimeInfo> {
  if (!runtimePromise) {
    runtimePromise = invoke<RuntimeInfo>('runtime_info').catch(() => browserFallback());
  }
  return runtimePromise;
}
