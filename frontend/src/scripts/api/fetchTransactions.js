import axios from "axios";

export default async function fetchTransactions() {
    const res = await axios.get(process.env.REACT_APP_API_URL + '/transaction');
    console.log(res.data);

    return res.data;
}