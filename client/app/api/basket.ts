import { v4 as uuidv4 } from "uuid";
import type {
  Basket,
  BasketItem,
  Product,
} from "./models";

const BASE =
  "https://alessandrorossi.tech:5001/api/";

export function createBasket(): Basket {
  const basket: Basket = {
    id: uuidv4(),
    items: [],
  };
  localStorage.setItem("basket_id", basket.id);
  return basket;
}

export async function getBasket(): Promise<Basket | null> {
  const basketId =
    localStorage.getItem("basket_id");
  if (!basketId) return null;
  const response = await fetch(
    BASE + `basket?id=${basketId}`
  );
  const basket: Basket = await response.json();
  console.log(basket);
  return basket;
}

export async function addProductToBasket(
  prod: Product
) {
  const basket =
    (await getBasket()) ?? createBasket();

  const bi: BasketItem = {
    id: prod.id,
    productName: prod.name,
    price: prod.price,
    quantity: 1,
    pictureUrl: prod.pictureUrl,
    brand: prod.productBrand,
    type: prod.productType,
  };
  basket.items.push(bi);

  try {
    await fetch(BASE + `basket`, {
      method: "POST",
      body: JSON.stringify(basket),
      headers: {
        "Content-Type": "application/json",
      },
    });
  } catch (error) {
    
  }
}
