import axios from "axios";

export default async function fetchTransactionEntities() {
    const res = await axios.get(process.env.REACT_APP_API_URL + '/transaction_entity');
    console.log(res.data);

    return res.data;
}