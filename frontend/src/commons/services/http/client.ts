import type { ApiSuccessResponse } from "./api-response";
import { HTTP_METHOD } from "./types";

class HttpClient {
  private readonly baseUrl =
    import.meta.env.VITE_API_HOST ?? "http://localhost:8080/api";

  private defaulteHeader(): HeadersInit {
    return {
      "content-type": "application/json",
    };
  }

  public async get<Response>(
    path: string,
    headers?: HeadersInit
  ): Promise<ApiSuccessResponse<Response>> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      method: HTTP_METHOD.GET,
      headers: {
        ...this.defaulteHeader(),
        ...headers,
      },
    });

    return response.json();
  }

  public async post<Body, Response>(
    path: string,
    body: Body,
    headers?: HeadersInit
  ): Promise<ApiSuccessResponse<Response>> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      method: HTTP_METHOD.POST,
      headers: {
        ...this.defaulteHeader(),
        ...headers,
      },
      body: JSON.stringify(body),
    });

    return response.json();
  }
}

export const httpClient = new HttpClient();
