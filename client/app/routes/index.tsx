import { Carousel } from "flowbite-react";

export default function Index() {
  return (
    <div className="h-56 sm:h-64 xl:h-80 2xl:h-96">
      <Carousel>
        <img src="/images/hero1.jpg" alt="..." />
        <img src="/images/hero2.jpg" alt="..." />
        <img src="/images/hero3.jpg" alt="..." />
      </Carousel>
      <div className="content-center">
        <h1 className="mx-auto mt-6 max-w-xl text-center text-xl text-black sm:max-w-3xl">
          Welcome to the shop!
        </h1>
      </div>
    </div>
  );
}
