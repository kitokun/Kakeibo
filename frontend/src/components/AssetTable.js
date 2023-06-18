import * as React from 'react';
import fetchAssets from '../scripts/api/fetchAssets';
import { useState, useEffect } from 'react';

export default function AssetTable() {
    const [amount, setRows] = useState([]);
    
    useEffect(() => {
        const get = async() => {
            const data = await fetchAssets();
            console.log(data)
            setRows(data.amount);
        };
        get();
    }, [])
    

    return (
        <div style={{ height: 200, width: 300}}>
            <span style={{color: "black"}}>
                {amount !== null ? `資産金額：${amount}` : 'データを取得中...'} 
            </span>
        </div>
    );
}