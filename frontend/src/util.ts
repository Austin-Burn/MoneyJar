/**
 * Formats a string of numbers into a phone number format.
 *
 * @param numbers - A string containing exactly 10 digits.
 * @returns The formatted phone number in the format (XXX) XXX-XXXX.
 */
export const formatPhoneNumber = (numbers: string): string => {
	return numbers.replace(/(\d{3})(\d{3})(\d{4})/, "($1) $2-$3");
};
