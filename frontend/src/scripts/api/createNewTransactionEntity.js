import axios from 'axios';

export default async function submitNewTransactionEntity(transactionEntity, transactionEntityType) {
    let data = {
        transaction_entity: transactionEntity,
        transaction_entity_type: transactionEntityType,
    };
    const res = await axios.post(process.env.REACT_APP_API_URL + '/transaction_entity', data);
    console.log(res);
}