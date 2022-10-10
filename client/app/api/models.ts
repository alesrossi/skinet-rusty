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

export interface Brand {
  id: number;
  name: string;
}

export interface Type {
  id: number;
  name: string;
}
