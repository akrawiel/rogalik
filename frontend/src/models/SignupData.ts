import LoginData from './LoginData';

export default interface SignupData extends LoginData {
  firstName: string;
  lastName: string;
}
