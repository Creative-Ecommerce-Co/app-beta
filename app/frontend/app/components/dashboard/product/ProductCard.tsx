import React from 'react';

interface ProductCardProps {
    imgSrc: string;
    imgAlt: string;
    prodCat: string;
    prodTitle: string;
    prodDesc: string;
    prodPrice: number;
    prodPriceOld?: number; // Optional for old price
    onAddToCart: () => void; // Function to handle adding to cart
}

const ProductCard: React.FC<ProductCardProps> = ({
    imgSrc,
    imgAlt,
    prodCat,
    prodTitle,
    prodDesc,
    prodPrice,
    prodPriceOld,
    onAddToCart,
}) => {
    return (
        <div className="bg-white rounded-xl shadow-lg p-4 m-4">
            <div className="img-container">
                <img
                    className="rounded-t-xl w-full h-48 object-cover"
                    src={imgSrc}
                    alt={imgAlt}
                />
            </div>
            <div className="product-info mt-4">
                <p className="text-xs uppercase text-gray-500">{prodCat}</p>
                <h1 className="text-2xl font-bold">{prodTitle}</h1>
                <p className="text-gray-700">{prodDesc}</p>
                <div className="my-2 flex items-center">
                    <p className="text-xl text-green-600 font-bold mr-2">${prodPrice.toFixed(2)}</p>
                    {prodPriceOld && (
                        <p className="text-sm line-through text-gray-500">${prodPriceOld.toFixed(2)}</p>
                    )}
                </div>
                <button
                    className="bg-green-600 text-white py-2 px-4 rounded-lg w-full"
                    onClick={onAddToCart}
                >
                    Add to Cart
                </button>
            </div>
        </div>
    );
};

export default ProductCard;