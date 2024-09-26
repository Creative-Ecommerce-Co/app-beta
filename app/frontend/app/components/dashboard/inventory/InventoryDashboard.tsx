// app/frontend/app/components/dashboard/inventory/InventoryDashboard.tsx

import React, { useState } from 'react';
import InventoryList from './InventoryList';
import InventoryItemForm from './InventoryItemForm';

const InventoryDashboard: React.FC = () => {
    const [items, setItems] = useState([]);
    const [editingItem, setEditingItem] = useState(null);

    const handleAddItem = (item) => {
        setItems([...items, { id: Date.now().toString(), ...item }]);
    };

    const handleEditItem = (id) => {
        const itemToEdit = items.find(item => item.id === id);
        setEditingItem(itemToEdit);
    };

    const handleUpdateItem = (updatedItem) => {
        setItems(items.map(item => (item.id === editingItem.id ? { ...item, ...updatedItem } : item)));
        setEditingItem(null);
    };

    const handleDeleteItem = (id) => {
        setItems(items.filter(item => item.id !== id));
    };

    return (
        <div>
            <InventoryItemForm onSubmit={editingItem ? handleUpdateItem : handleAddItem} initialData={editingItem} />
            <InventoryList items={items} onEdit={handleEditItem} onDelete={handleDeleteItem} />
        </div>
    );
};

export default InventoryDashboard;