import type {
  Brand,
  PaginatedResponse,
  Product,
  ProductsParams,
  Type,
} from "./models";

const DEFAULT_PAGE_INDEX = 1;
const DEFAULT_PAGE_SIZE = 6;

export async function getProducts(
  params: ProductsParams
) {
  process.env.NODE_TLS_REJECT_UNAUTHORIZED = "0";
  console.log("params: ", params);
  let url = new URL(
    "https://alessandrorossi.tech:5001/api/products"
  );
  for (const [param, value] of Object.entries(
    params
  )) {
    if (value !== null) {
      url.searchParams.append(param, value);
    } else {
      if (param === "pageIndex") {
        url.searchParams.append(
          "pageIndex",
          DEFAULT_PAGE_INDEX.toString()
        );
      }
      if (param === "pageSize") {
        url.searchParams.append(
          "pageSize",
          DEFAULT_PAGE_SIZE.toString()
        );
      }
    }
  }
  console.log("url ", url);
  const response = await fetch(url);

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

export async function getSortOptions() {
  return [
    { name: "Alphabetical", value: "name.desc" },
    {
      name: "Price: Low to high",
      value: "price.asc",
    },
    {
      name: "Price: High to low",
      value: "price.desc",
    },
  ];
}
