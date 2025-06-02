import {Schema} from "effect";

export const GetEventRequest = Schema.Struct({
	id: Schema.UUID,
	ownerId: Schema.UUID,
	name: Schema.String,
	description: Schema.OptionFromNullishOr(Schema.String, null),
	eventDate: Schema.OptionFromNullishOr(Schema.String, null),
	reoccuring: Schema.Boolean,
	reoccuringInterval: Schema.OptionFromNullishOr(Schema.String, null),
	finalDate: Schema.OptionFromNullishOr(Schema.String, null),
});
