import {type HttpClient, HttpClientRequest, HttpClientResponse,} from "@effect/platform";
import {Effect, type Schema} from "effect";

/**
 * Formats a string of numbers into a phone number format.
 *
 * @param numbers - A string containing exactly 10 digits.
 * @returns The formatted phone number in the format (XXX) XXX-XXXX.
 */
export const formatPhoneNumber = (numbers: string): string => {
	return numbers.replace(/(\d{3})(\d{3})(\d{4})/, "($1) $2-$3");
};

/**
 * Creates a function that makes an API request.
 *
 * @param httpClient - The HTTP client to use to make the request.
 * @returns A function that makes an API request.
 *
 * The returned function returns an effect that executes the request and returns the response body
 * parsed according to the provided JSON schema.
 */
export function createApiRequestFunction(httpClient: HttpClient.HttpClient) {
	return <A, I, R>(
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
}
