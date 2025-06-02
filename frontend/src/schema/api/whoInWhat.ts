import {Schema} from "effect";

export const GetEventsFromUserRequest = Schema.Struct({
	events: Schema.Array(Schema.UUID),
});
