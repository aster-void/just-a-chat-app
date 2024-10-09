import { myData } from "$lib/api/user";

/** @type import("./@types").PageLoad **/
export function load({ param }) {
	console.log(param.id);
	return {
		data: myData,
	};
}
