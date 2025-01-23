export interface UserPayload {
  userName: string;
  email: string;
  password: string;
  confirmPassword: string;
  firstName: string;
  lastName: string;
}

export interface User {
  id: string;
  userName: string;
  email: string;
  firstName: string;
  lastName: string;
  createdAt: Date;
}
