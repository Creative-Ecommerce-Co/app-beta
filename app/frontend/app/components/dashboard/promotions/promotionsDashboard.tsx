// app/frontend/app/components/dashboard/promotions/PromotionsDashboard.tsx

import React, { useState } from 'react';
import PromotionsList from './PromotionsList';
import PromotionForm from './PromotionForm';

const PromotionsDashboard: React.FC = () => {
    const [promotions, setPromotions] = useState([]);
    const [editingPromotion, setEditingPromotion] = useState(null);

    const handleAddPromotion = (promotion) => {
        setPromotions([...promotions, { id: Date.now().toString(), ...promotion }]);
    };

    const handleEditPromotion = (id) => {
        const promotionToEdit = promotions.find(promotion => promotion.id === id);
        setEditingPromotion(promotionToEdit);
    };

    const handleUpdatePromotion = (updatedPromotion) => {
        setPromotions(promotions.map(promotion => (promotion.id === editingPromotion.id ? { ...promotion, ...updatedPromotion } : promotion)));
        setEditingPromotion(null);
    };

    const handleDeletePromotion = (id) => {
        setPromotions(promotions.filter(promotion => promotion.id !== id));
    };

    return (
        <div>
            <PromotionForm onSubmit={editingPromotion ? handleUpdatePromotion : handleAddPromotion} initialData={editingPromotion} />
            <PromotionsList promotions={promotions} onEdit={handleEditPromotion} onDelete={handleDeletePromotion} />
        </div>
    );
};

export default PromotionsDashboard;