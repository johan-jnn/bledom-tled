import { invoke } from "@tauri-apps/api/core";
import type { Device } from "../../types/device";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    device: await invoke<Device>("device_init", { force: false }),
  };
};
