import type {
  Brand,
  PaginatedResponse,
  Product,
  Type,
} from "./models";

const DEFAULT_PAGE_INDEX = 1;
const DEFAULT_PAGE_SIZE = 6;

export async function getProducts(
  search: string | null
) {
  process.env.NODE_TLS_REJECT_UNAUTHORIZED = "0";
  let query = "";
  if (search) {
    query = `https://alessandrorossi.tech:5001/api/products?sort=name.asc&name=${search}&pageIndex=${DEFAULT_PAGE_INDEX}&pageSize=${DEFAULT_PAGE_SIZE}`;
  } else {
    query = `https://alessandrorossi.tech:5001/api/products?sort=name.asc&pageIndex=${DEFAULT_PAGE_INDEX}&pageSize=${DEFAULT_PAGE_SIZE}`;
  }
  const response = await fetch(query);

  const products: PaginatedResponse =
    await response.json();

  return products;
}

export async function getProductById(
  productId: number
): Promise<Product> {
  const response = await fetch(
    `https://alessandrorossi.tech:5001/api/products/${productId}`
  );

  return await response.json();
}

export async function getBrands(): Promise<
  Brand[]
> {
  process.env.NODE_TLS_REJECT_UNAUTHORIZED = "0";

  const response = await fetch(
    "https://alessandrorossi.tech:5001/api/products/brands"
  );

  return response.json();
}

export async function getTypes(): Promise<
  Type[]
> {
  process.env.NODE_TLS_REJECT_UNAUTHORIZED = "0";

  const response = await fetch(
    "https://alessandrorossi.tech:5001/api/products/types"
  );

  return response.json();
}
