import {
	CreateUserRequest,
	DeleteUserRequest,
	GetEmailRequest,
	GetNameRequest,
	GetUserInfoRequest,
	LoginRequest,
	UpdateEmailRequest,
	UpdateNameRequest,
	UpdatePhoneRequest,
} from "@/schema/api/user.ts";
import {HttpClient, HttpClientRequest, HttpClientResponse,} from "@effect/platform";
import {Effect, type Schema} from "effect";

/**
 * The UserApi service provides API endpoints for User-related
 * operations.
 *
 * See the {@link UserApi} docs for more information.
 */
export class UserApi extends Effect.Service<UserApi>()("UserApi", {
	accessors: true,
	effect: Effect.gen(function* () {
		yield* Effect.logTrace("Setting up UserApi");

		const httpClient = yield* HttpClient.HttpClient.pipe(
			Effect.map(HttpClient.filterStatusOk),
		);

		const apiRequest = <A, I, R>(
			path: string,
			body: unknown,
			schema: Schema.Schema<A, I, R>,
		) =>
			HttpClientRequest.post(path).pipe(
				HttpClientRequest.prependUrl("/api"),
				HttpClientRequest.bodyUnsafeJson(body),
				httpClient.execute,
				Effect.andThen(HttpClientResponse.schemaBodyJson(schema)),
			);

		return {
			createUser: (name: string, email: string, password: string) =>
				apiRequest("/CreateUser", { name, email, password }, CreateUserRequest),

			updateName: (id: string, name: string) =>
				apiRequest("/UpdateName", { id, name }, UpdateNameRequest),
			updateEmail: (id: string, email: string) =>
				apiRequest("/UpdateEmail", { id, email }, UpdateEmailRequest),
			updatePhone: (id: string, phone: string) =>
				apiRequest("/UpdatePhone", { id, phone }, UpdatePhoneRequest),

			getUserInfo: (id: string) =>
				apiRequest("/UserGetAll", { id }, GetUserInfoRequest),
			getName: (id: string) => apiRequest("/GetName", { id }, GetNameRequest),
			getEmail: (id: string) =>
				apiRequest("/GetEmail", { id }, GetEmailRequest),

			deleteUser: (id: string) =>
				apiRequest("/DeleteUser", { id }, DeleteUserRequest),

			login: (email: string, password: string) =>
				apiRequest("/Login", { email, password }, LoginRequest),
		};
	}),
}) {}
