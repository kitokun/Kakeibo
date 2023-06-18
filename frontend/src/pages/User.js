import React from "react";
import DataTable from '../components/TransactionsTable';
import TransactionsDialog from '../components/TransactionsDialog';
import SuppliersDialog from '../components/SuppliersDialog'
import AssetTable from '../components/AssetTable';

function User() {
  return (
    <div className="App">
      <body className='App-body'>
        <AssetTable />
        <DataTable />
        <TransactionsDialog />
        <SuppliersDialog />
      </body>
    </div>
  );
}
export default User;