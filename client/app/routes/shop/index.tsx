import type { LoaderFunction } from "@remix-run/node";
import {
  Form,
  useLoaderData,
} from "@remix-run/react";
import type {
  Brand,
  PaginatedResponse,
  Product,
  Type,
} from "~/api/models";
import {
  getBrands,
  getProducts,
  getTypes,
} from "~/api/products";

type InitData = {
  brands: Brand[];
  types: Type[];
  products: PaginatedResponse;
};

const filtering =
  "text-xl cursor-pointer hover:bg-orange-500 py-3 pl-3";
const base = "https://alessandrorossi.tech:5001/";

export const loader: LoaderFunction =
  async () => {
    const brands = await getBrands();
    const types = await getTypes();
    const products = await getProducts(null);
    return {
      brands: brands,
      types: types,
      products: products,
    };
  };

export default function Shop() {
  const initData = useLoaderData<InitData>();

  return (
    <div className="grid grid-cols-6 px-6 mx-60">
      <div className="col-span-1">
        <div className="py-3 ml-4">
          <h1 className="text-xl text-orange-500 pl-3">
            Brands
          </h1>
          <ul>
            <li className={filtering}>All</li>
            {initData.brands.map((brand) => (
              <li
                className={filtering}
                key={brand.id}
              >
                {brand.name}
              </li>
            ))}
          </ul>
        </div>
        <div className="py-3 ml-4">
          <h1 className="text-xl text-orange-500 pl-3">
            Types
          </h1>
          <ul>
            <li className={filtering}>All</li>
            {initData.types.map((type) => (
              <h1
                className={filtering}
                key={type.id}
              >
                {type.name}
              </h1>
            ))}
          </ul>
        </div>
      </div>
      <div className="flex flex-col col-span-5">
        <div className="flex justify-between py-4 align-middle">
          <div className="align-middle">
            Showing{" "}
            <strong>
              {initData.products.pageIndex} -{" "}
              {initData.products.pageSize}
            </strong>{" "}
            of{" "}
            <strong>
              {initData.products.count}
            </strong>
          </div>
          <Form reloadDocument className="py-5">
            <div className="flex gap-3">
              <input
                className="border border-black rounded-lg"
                placeholder="Search..."
              ></input>
              <button
                type="submit"
                className="bg-white border-2 border-orange-400 hover:bg-orange-500 text-black py-2 px-4 rounded-lg"
              >
                Search
              </button>
              <button className="bg-white border-2 border-green-400 hover:bg-green-400 text-black py-2 px-4 rounded-lg">
                Reset
              </button>
            </div>
          </Form>
        </div>
        <div className="grid grid-cols-3 gap-16">
          {initData.products.data.map(
            (product) => (
              <div
                className="max-w-sm rounded overflow-hidden shadow-lg"
                key={product.id}
              >
                <img
                  className="w-auto bg-cyan-600"
                  src={base + product.pictureUrl}
                  alt={product.name}
                />
                <div className="px-6 py-4">
                  <div className="font-bold text-orange-500 text-xl mb-2">
                    {product.name}
                  </div>
                  <p className="text-gray-700 text-xl">
                    {"€" + product.price}
                  </p>
                </div>
              </div>
            )
          )}
        </div>
      </div>
    </div>
  );
}
