import axios from "axios";

export default async function fetchAssets() {
    const res = await axios.get(process.env.REACT_APP_API_URL + '/asset');
    console.log(res.data);

    return res.data;
}