export interface APIResponse {
  status: string;
  message: string | null;
  data: any;
  amount: number | null;
}