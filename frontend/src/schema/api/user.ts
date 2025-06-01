import {Schema} from "effect";

export const CreateUserRequest = Schema.Struct({
	id: Schema.UUID,
});

export const UpdateNameRequest = Schema.Struct({});

export const UpdateEmailRequest = Schema.Struct({});

export const UpdatePhoneRequest = Schema.Struct({});

export const GetNameRequest = Schema.Struct({
	name: Schema.String,
});

export const GetEmailRequest = Schema.Struct({
	email: Schema.String,
});

export const GetPhoneRequest = Schema.Struct({
	phone: Schema.String,
});

export const GetUserInfoRequest = Schema.Struct({
	id: Schema.UUID,
	name: Schema.String,
	email: Schema.String,
	phone: Schema.String,
});

export const DeleteUserRequest = Schema.Struct({});

export const LoginRequest = Schema.Struct({});
