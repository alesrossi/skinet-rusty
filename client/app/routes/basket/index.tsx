import { Link } from "@remix-run/react";
import { Table } from "flowbite-react";
import { Basket } from "~/api/models";

const base = "https://alessandrorossi.tech:5001/";

export default function Basket() {
  return (
    <div className="px-56">
      <Table hoverable={true}>
        <Table.Head
          style={{
            background: "#e9ecef",
            marginBottom: "10px",
            marginTop: "10px",
          }}
        >
          <Table.HeadCell>Product</Table.HeadCell>
          <Table.HeadCell>Price</Table.HeadCell>
          <Table.HeadCell>
            Quantity
          </Table.HeadCell>
          <Table.HeadCell>Total</Table.HeadCell>
          <Table.HeadCell>Remove</Table.HeadCell>
        </Table.Head>
        <Table.Body className="divide-y">
          <Table.Row className="bg-white dark:border-gray-700 dark:bg-gray-800">
            <Table.Cell className="whitespace-nowrap font-medium text-gray-900 dark:text-white">
              <div className="flex gap-6 align-middle">
                <img
                  className="max-w-[10%] max-h-[10%]"
                  src="https://alessandrorossi.tech:5001/images/products/boot-ang2.png"
                  alt="boots"
                ></img>
                <div className="flex flex-col justify-center items-center">
                  <p className="text-lg mb-0">
                    Angular Blue Boots
                  </p>
                </div>
              </div>
            </Table.Cell>
            <Table.Cell>
              <div className="flex flex-col justify-center items-center">
                <p className="text-lg mb-0">
                  100€
                </p>
              </div>
            </Table.Cell>
            <Table.Cell>
              {" "}
              <div className="flex justify-center items-center gap-2">
                <button
                  className="w-8 h-8 rounded-full font-bold
                       bg-orange-500 text-2xl hover:bg-red-500 text-white"
                >
                  -
                </button>
                <h1 className="text-2xl font-bold">
                  1
                </h1>
                <button
                  className="w-8 h-8 rounded-full font-bold
                       bg-orange-500 text-2xl hover:bg-red-500 text-white"
                >
                  +
                </button>
              </div>
            </Table.Cell>
            <Table.Cell>
              <div className="flex flex-col justify-center items-center">
                <p className="text-lg mb-0">
                  200€
                </p>
              </div>
            </Table.Cell>
            <Table.Cell>
              <div className="flex flex-col justify-center items-center">
                <Link to="/">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    strokeWidth={1.5}
                    stroke="red"
                    className="w-10 h-10"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"
                    />
                  </svg>
                </Link>
              </div>
            </Table.Cell>
          </Table.Row>
          <Table.Row className="bg-white dark:border-gray-700 dark:bg-gray-800">
            <Table.Cell className="whitespace-nowrap font-medium text-gray-900 dark:text-white">
              Microsoft Surface Pro
            </Table.Cell>
            <Table.Cell>White</Table.Cell>
            <Table.Cell>Laptop PC</Table.Cell>
            <Table.Cell>$1999</Table.Cell>
            <Table.Cell>
              <a
                href="/tables"
                className="font-medium text-blue-600 hover:underline dark:text-blue-500"
              >
                Edit
              </a>
            </Table.Cell>
          </Table.Row>
          <Table.Row className="bg-white dark:border-gray-700 dark:bg-gray-800">
            <Table.Cell className="whitespace-nowrap font-medium text-gray-900 dark:text-white">
              Magic Mouse 2
            </Table.Cell>
            <Table.Cell>Black</Table.Cell>
            <Table.Cell>Accessories</Table.Cell>
            <Table.Cell>$99</Table.Cell>
            <Table.Cell>
              <a
                href="/tables"
                className="font-medium text-blue-600 hover:underline dark:text-blue-500"
              >
                Edit
              </a>
            </Table.Cell>
          </Table.Row>
        </Table.Body>
      </Table>
    </div>
  );
}


