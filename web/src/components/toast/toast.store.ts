import { type Writable, writable } from "svelte/store";

export type ToastElement = {
	id: string;
	message: string;
	kind: ToastKind;
};
export const toasts: Writable<ToastElement[]> = writable([]);

export type ToastKind = "default" | "success" | "info" | "warning" | "error";
export function pushToast(message: string, kind: ToastKind, timeout: number) {
	const id = Math.random().toString();
	toasts.update((prev) => [...prev, { id, message, kind }]);
	setTimeout(() => {
		toasts.update((prev) => prev.filter((elem) => elem.id !== id));
	}, timeout);
}
