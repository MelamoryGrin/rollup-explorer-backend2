import useSWR from "swr";

const fetcher = (url: string) => fetch(url).then((r) => r.json());

export function useL1Tps() {
  const { data, error } = useSWR(
    `${process.env.NEXT_PUBLIC_BASE_API_URL}/l1_tps`,
    fetcher,
    {
      refreshInterval: 3000,
    }
  );
  return {
    l1Tps: data,
    isLoading: !error && !data,
    isError: error,
  };
}

export function useL2Tps() {
  const { data, error } = useSWR(
    `${process.env.NEXT_PUBLIC_BASE_API_URL}/l2_tps`,
    fetcher,
    {
      refreshInterval: 3000,
    }
  );
  return {
    l2Tps: data,
    isLoading: !error && !data,
    isError: error,
  };
}
