import http from "k6/http";
import { sleep, check } from "k6";

export const options = {
	vus: 10, // jumlah virtual users
	duration: "30s", // lama tes
};

export default function () {
	const res = http.get("http://localhost:3000/orders");

	check(res, {
		"status 200": (r) => r.status === 200,
		"response time < 500ms": (r) => r.timings.duration < 500,
	});

	// sleep(1); // jeda antar request
}
