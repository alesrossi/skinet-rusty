export type PaginatedResponse = {
  pageIndex: number;
  pageSize: number;
  count: number;
  data: Product[];
};

export type Product = {
  id: number;
  name: string;
  description: string;
  price: number;
  pictureUrl: string;
  productType: string;
  productBrand: string;
};

export type Brand = {
  id: number;
  name: string;
};

export type Type = {
  id: number;
  name: string;
};

export type ProductsParams = {
  name: string | null;
  sort: string | null;
  brandId: string | null;
  typeId: string | null;
  pageIndex: string | null;
  pageSize: string | null;
};

export type Basket = {
  id: string;
  items: BasketItem[];
  clientSecret?: string;
  paymentIntentId?: string;
  deliveryMethodId?: number;
  shippingPrice?: number;
};

export type BasketItem = {
  id: number;
  productName: string;
  price: number;
  quantity: number;
  pictureUrl: string;
  brand: string;
  type: string;
};
