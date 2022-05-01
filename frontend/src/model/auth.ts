import {
  IsEqualToField,
  PasswordValidator,
  UsernameValidator,
} from "../common";

export class LoginInput {
  @UsernameValidator()
  identity!: string;
  @PasswordValidator()
  password!: string;
}

export class ChangePasswordForm {
  @PasswordValidator()
  oldPassword!: string;
  @PasswordValidator()
  newPassword!: string;
  @IsEqualToField<ChangePasswordForm>(`newPassword`)
  @PasswordValidator()
  newPassword2!: string;
}
