import type {
  LinksFunction,
  MetaFunction,
} from "@remix-run/node";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { Navbar } from "flowbite-react";
import styles from "./styles/app.css";

type ChildrenProp = {
  children: ReactNode;
};

export const links: LinksFunction = () => {
  return [{ rel: "stylesheet", href: styles }];
};

export const meta: MetaFunction = () => {
  return {
    title: "Skinet-Rusty",
    description: "A description",
  };
};

export default function App() {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="width=device-width,initial-scale=1"
        />
        <Meta />
        <Links />
      </head>
      <body>
        <Layout>
          <Outlet />
        </Layout>
        <ScrollRestoration />
        <Scripts />
        {process.env.NODE_ENV ===
          "development" && <LiveReload />}
      </body>
    </html>
  );
}

// A standard layout that will be the same for each route
export function Layout({
  children,
}: ChildrenProp) {
  return (
    <>
      <Navbar fluid={true} rounded={true}>
        <Navbar.Brand href="/">
          <img
            src="/images/logo.png"
            className="mr-3  h-13"
            alt="Skinet Rusty"
          />
        </Navbar.Brand>
        <Navbar.Toggle />

        <Navbar.Collapse>
          <Navbar.Link href="/" active={true}>
            HOME
          </Navbar.Link>
          <Navbar.Link href="/shop">
            SHOP
          </Navbar.Link>
          <Navbar.Link href="/errors">
            ERRORS
          </Navbar.Link>
        </Navbar.Collapse>

        <Navbar.Collapse>
          <Navbar.Link href="/">CART</Navbar.Link>
          <Navbar.Link href="/shop">
            LOGIN
          </Navbar.Link>
          <Navbar.Link href="/errors">
            SIGN-UP
          </Navbar.Link>
        </Navbar.Collapse>
      </Navbar>
      {children}
    </>
  );
}

export function ErrorBoundary({ error }) {
  console.error(error);
  return (
    <html>
      <head>
        <title>Oh no!</title>
        <Meta />
        <Links />
      </head>
      <body>
        {/* add the UI you want your users to see */}
        {error.message}
        <Scripts />
      </body>
    </html>
  );
}
