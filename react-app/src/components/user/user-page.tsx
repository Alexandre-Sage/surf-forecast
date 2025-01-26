import { User } from "../../types/user.type";
import { Page } from "../ui";

export interface UserPageProps {
  user: User;
}
export const UserPage = () => {
  return <Page title={props.user.userName}>Hello world</Page>;
};
