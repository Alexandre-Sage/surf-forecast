export interface ApiErrorResponse {
  success: false;
  error: string;
}

export interface ApiSuccessResponse<Return> {
  payload: Return;
  success: true;
}
