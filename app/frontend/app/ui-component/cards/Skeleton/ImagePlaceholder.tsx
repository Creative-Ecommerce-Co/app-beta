// material-ui
import Skeleton, { SkeletonProps } from '@mui/material/Skeleton';

// ==============================|| SKELETON IMAGE CARD ||============================== //

const ImagePlaceholder: React.FC<SkeletonProps> = (props) => <Skeleton variant="rectangular" {...props} animation="wave" />;

export default ImagePlaceholder;