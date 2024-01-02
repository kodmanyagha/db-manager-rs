import { invoke } from '@tauri-apps/api';

export type InvokeFuncParamsType = null | undefined | object | string | number | bigint;

export async function invokeTauri(fn: string, data: InvokeFuncParamsType) {
  let result: unknown = undefined;

  try {
    result = await invoke('invoke_handler', {
      func: fn,
      data: JSON.stringify(data),
    });

    return result;
  } catch (e) {
    if (e instanceof Error) {
      result = 'ERROR: ' + e.message;
    } else {
      result = 'Unknown error occured' + e;
    }
  }

  return result;
}
