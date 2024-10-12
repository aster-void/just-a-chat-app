type Ok<T> = { ok: true; val: T };
type Err = { ok: false; err: unknown };

export type Result<T> = Ok<T> | Err;

export function Ok<T>(val: T): Ok<T> {
	return {
		ok: true,
		val,
	};
}

export function Err(err: unknown): Err {
	return {
		ok: false,
		err,
	};
}
