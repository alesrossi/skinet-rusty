import type { LoaderFunction } from "@remix-run/node";
import {
  Form,
  useLoaderData,
} from "@remix-run/react";
import { useState } from "react";
import invariant from "tiny-invariant";
import type { Product } from "~/api/models";
import { getProductById } from "~/api/products";

const base = "https://alessandrorossi.tech:5001/";

export const loader: LoaderFunction = async ({
  params,
}) => {
  invariant(
    params.productId,
    "Expected a product id"
  );

  const product = await getProductById(
    +params.productId
  );

  return product;
};

export default function ProductDetails() {
  const product = useLoaderData<Product>();
  let [count, setCount] = useState(1);

  return (
    <div className="flex flex-col align-middle justify-center">
      <div className="flex gap-4 px-6">
        <img
          className="w-auto"
          src={base + product.pictureUrl}
          alt={product.name}
        />

        <div className="flex flex-col">
          <h1 className="text-4xl">
            {product.name}
          </h1>
          <h1 className="text-4xl py-4">
            {product.price + "€"}
          </h1>
          <Form className="flex py-6">
            <div className="flex gap-8">
              <button
                className="w-16 h-16 rounded-full 
                       bg-orange-500 text-5xl hover:bg-red-500 text-white"
                onClick={() =>
                  count - 1 !== 0
                    ? setCount(count - 1)
                    : count
                }
              >
                -
              </button>
              <h1 className="text-4xl mt-3">
                {count}
              </h1>
              <button
                className="w-16 h-16 rounded-full 
                       bg-orange-500 text-5xl hover:bg-red-500 text-white"
                onClick={() =>
                  setCount(count + 1)
                }
              >
                +
              </button>
            </div>
            <button>Add to cart</button>
          </Form>
        </div>
      </div>
      <div className="py-4">
        <p>{product.description}</p>
      </div>
    </div>
  );
}
