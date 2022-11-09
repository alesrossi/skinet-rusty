import {
  LoaderFunction,
  redirect,
} from "@remix-run/node";
import {
  Form,
  Link,
  useLoaderData,
  useLocation,
  useNavigate,
  useSearchParams,
} from "@remix-run/react";
import { Pagination } from "flowbite-react";
import { useState } from "react";
import type {
  Brand,
  PaginatedResponse,
  Type,
  ProductsParams,
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
  "text-xl cursor-pointer py-3 pl-3 rounded-lg text-left hoverable";
const base = "https://alessandrorossi.tech:5001/";

function onPageChange(page: number) {}

export const loader: LoaderFunction = async ({
  request,
}) => {
  const paramsFromUrl = new URL(request.url)
    .searchParams;

  const params: ProductsParams = {
    name: paramsFromUrl.get("name"),
    sort: paramsFromUrl.get("sort"),
    brandId: paramsFromUrl.get("brand"),
    typeId: paramsFromUrl.get("type"),
    pageIndex: paramsFromUrl.get("Index"),
    pageSize: paramsFromUrl.get("Size"),
  };

  let brands = await getBrands();
  let types = await getTypes();
  let products = await getProducts(params);

  return {
    brands: brands,
    types: types,
    products: products,
  };
};

export default function Shop() {
  const initData = useLoaderData<InitData>();
  const [searchParams] = useSearchParams();

  const changeActive = (
    doc: Document,
    name: string,
    listName: string
  ) => {
    for (const child of doc.getElementById(
      listName
    )!.children) {
      if (child.classList.contains("active")) {
        child.classList.remove("active");
      }
    }

    let el = doc.getElementById(name)!;
    el.classList.add("active");
  };

  const setBrandParam = (id: number) => {
    if (id === 0) {
      console.log(searchParams.toString());
      searchParams.delete("brand");
      return `?${searchParams.toString()}`;
    }
    let copied = new URLSearchParams(
      searchParams.toString()
    );
    copied.set("brand", id.toString());
    return `?${copied.toString()}`;
  };

  const setTypeParam = (id: number) => {
    if (id === 0) {
      console.log(searchParams.toString());
      searchParams.delete("type");
      return `?${searchParams.toString()}`;
    }
    let copied = new URLSearchParams(
      searchParams.toString()
    );
    copied.set("type", id.toString());
    return `?${copied.toString()}`;
  };

  return (
    <Form
      className="grid grid-cols-6 px-6 mx-60"
      method="get"
    >
      <div className="col-span-1">
        <h1>TEST</h1>
        <div className="py-3 mx-4">
          <h1 className="text-xl text-orange-500 pl-3">
            Brands
          </h1>
          <div
            className="flex flex-col"
            id="brandsList"
          >
            <Link
              to={setBrandParam(0)}
              className={filtering + " active"}
              onClick={() =>
                changeActive(
                  document,
                  "brandsAll",
                  "brandsList"
                )
              }
              id="brandsAll"
              type="submit"
            >
              All
            </Link>
            <input
              type="hidden"
              name={
                searchParams.get("brand")
                  ? "brand"
                  : ""
              }
              value={
                searchParams.get("brand")
                  ? searchParams.get("brand")!
                  : ""
              }
            ></input>
            {initData.brands.map((brand) => (
              <Link
                to={setBrandParam(brand.id)}
                className={filtering}
                onClick={() =>
                  changeActive(
                    document,
                    brand.name,
                    "brandsList"
                  )
                }
                key={brand.id}
                id={brand.name}
                type="submit"
              >
                {brand.name}
              </Link>
            ))}
          </div>
        </div>
        <div className="py-3 mx-4">
          <h1 className="text-xl text-orange-500 pl-3">
            Types
          </h1>
          <div
            className="flex flex-col"
            id="typesList"
          >
            <Link
              to={setTypeParam(0)}
              className={filtering + " active"}
              onClick={() =>
                changeActive(
                  document,
                  "typesAll",
                  "typesList"
                )
              }
              id="typesAll"
              type="submit"
            >
              All
            </Link>
            <input
              type="hidden"
              name={
                searchParams.get("type")
                  ? "type"
                  : ""
              }
              value={
                searchParams.get("type")
                  ? searchParams.get("type")!
                  : ""
              }
            ></input>
            {initData.types.map((type) => (
              <Link
                to={setTypeParam(type.id)}
                className={filtering}
                onClick={() =>
                  changeActive(
                    document,
                    type.name,
                    "typesList"
                  )
                }
                key={type.id}
                id={type.name}
                type="submit"
              >
                {type.name}
              </Link>
            ))}
          </div>
        </div>
      </div>
      <div className="flex flex-col col-span-5 align-middle">
        <div className="flex justify-between py-4 align-middle">
          <div className="py-7 ml-1 align-middle">
            Showing{" "}
            <strong>
              {initData.products.count > 0
                ? initData.products.pageIndex
                : 0}{" "}
              -{" "}
              {initData.products.pageSize >
              initData.products.count
                ? initData.products.count
                : initData.products.pageSize}
            </strong>{" "}
            of{" "}
            <strong>
              {initData.products.count}
            </strong>
          </div>
          <div className="flex gap-3 py-5">
            <input
              type="text"
              name="name"
              className="border border-black rounded-lg"
              placeholder="Search..."
            ></input>
            <button
              type="submit"
              value="name"
              className="bg-white border-2 border-orange-400 hover:bg-orange-500 text-black py-2 px-4 rounded-lg"
            >
              Search
            </button>
            <Link
              reloadDocument
              to="/shop"
              className="bg-white border-2 border-green-400 hover:bg-green-400 text-black py-2 px-4 rounded-lg"
            >
              Reset
            </Link>
          </div>
        </div>
        <div className="grid grid-cols-3 gap-16">
          {initData.products.data.map(
            (product) => (
              <div
                className="max-w-sm rounded overflow-hidden shadow-lg"
                key={product.id}
              >
                <div className="relative">
                  <div className="group absolute inset-0 flex gap-8 justify-center items-center">
                    <button className="invisible group-hover:visible text-xl cursor-pointer p-3 rounded-lg text-center bg-orange-500">
                      CART
                    </button>
                    <Link
                      to={`/shop/${product.id}`}
                      className="invisible group-hover:visible text-xl cursor-pointer p-3 rounded-lg text-center bg-orange-500"
                    >
                      VIEW
                    </Link>
                  </div>
                  <img
                    className="w-auto bg-cyan-600"
                    src={
                      base + product.pictureUrl
                    }
                    alt={product.name}
                  />
                </div>
                <div className="px-6 py-4">
                  <div className="font-bold text-orange-500 text-xl mb-2">
                    {product.name}
                  </div>
                  <p className="text-gray-700 text-xl">
                    {product.price + "€"}
                  </p>
                </div>
              </div>
            )
          )}
        </div>
        <div className="mx-auto py-5">
          <Pagination
            currentPage={
              initData.products.pageIndex
            }
            onPageChange={onPageChange}
            showIcons={true}
            totalPages={
              initData.products.count /
              initData.products.pageSize
            }
          />
        </div>
      </div>
    </Form>
  );
}
