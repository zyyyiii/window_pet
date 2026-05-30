import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SystemInfo } from "../types/events";

export function useSystemInfo() {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchSystemInfo = async () => {
      try {
        const info = await invoke("get_system_info") as SystemInfo;
        setSystemInfo(info);
        setError(null);
      } catch (err) {
        setError(String(err));
      } finally {
        setIsLoading(false);
      }
    };

    fetchSystemInfo();

    const interval = setInterval(fetchSystemInfo, 5000);

    return () => clearInterval(interval);
  }, []);

  return { systemInfo, isLoading, error };
}