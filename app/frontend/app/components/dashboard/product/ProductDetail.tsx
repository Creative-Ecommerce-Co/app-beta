// app/frontend/app/components/product/ProductDetail.tsx

import React from 'react';
import { Box, Button, Typography } from '@mui/material';

interface ProductDetailProps {
    imgSrc: string;
    imgAlt: string;
    prodTitle: string;
    prodDesc: string;
    prodPrice: number;
    onAddToCart: () => void;
}

const ProductDetail: React.FC<ProductDetailProps> = ({
    imgSrc,
    imgAlt,
    prodTitle,
    prodDesc,
    prodPrice,
    onAddToCart,
}) => {
    return (
        <Box className="bg-white rounded-xl shadow-lg p-4 m-4">
            <div className="img-container">
                <img
                    className="rounded-t-xl w-full h-48 object-cover"
                    src={imgSrc}
                    alt={imgAlt}
                />
            </div>
            <div className="product-info mt-4">
                <Typography variant="h4" className="font-bold">
                    {prodTitle}
                </Typography>
                <Typography variant="body1" className="text-gray-700">
                    {prodDesc}
                </Typography>
                <Typography variant="h5" className="text-green-600 font-bold my-2">
                    ${prodPrice.toFixed(2)}
                </Typography>
                <Button
                    variant="contained"
                    color="primary"
                    onClick={onAddToCart}
                    className="w-full"
                >
                    Add to Cart
                </Button>
            </div>
        </Box>
    );
};

export default ProductDetail;