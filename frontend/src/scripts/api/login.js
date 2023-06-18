import axios from "axios";

export default async function login(email, password) {
    const res = await axios.post(process.env.REACT_APP_API_URL + "/login", {
        email: email,
        password: password,
    });
    console.log(res);

    return res.data;
}
