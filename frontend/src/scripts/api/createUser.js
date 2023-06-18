import axios from "axios";

export default async function createUser(username) {
    let data = {
        username: username,
    };
    const res = await axios.post(process.env.REACT_APP_API_URL + '/user', data);
    console.log(res);
}