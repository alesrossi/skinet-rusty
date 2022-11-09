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
import type { ReactNode } from "react";
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
      <Navbar
        fluid={true}
        rounded={true}
        style={{
          position: "fixed",
          top: "0",
          zIndex: "1",
          width: "100%",
        }}
      >
        <Navbar.Brand href="/">
          <img
            src="/images/logo.png"
            className="mr-3  h-13"
            alt="Skinet Rusty"
          />
        </Navbar.Brand>
        <Navbar.Toggle />

        <Navbar.Collapse>
          <Navbar.Link
            style={{ fontSize: "22px" }}
            href="/"
            active={true}
          >
            HOME
          </Navbar.Link>
          <Navbar.Link
            style={{ fontSize: "22px" }}
            href="/shop"
          >
            SHOP
          </Navbar.Link>
          <Navbar.Link
            style={{ fontSize: "22px" }}
            href="/errors"
          >
            ERRORS
          </Navbar.Link>
        </Navbar.Collapse>

        <Navbar.Collapse>
          <Navbar.Link
            href="/basket"
            aria-label="CART"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="currentColor"
              className="w-10 h-10 -mt-2"
            >
              <path d="M2.25 2.25a.75.75 0 000 1.5h1.386c.17 0 .318.114.362.278l2.558 9.592a3.752 3.752 0 00-2.806 3.63c0 .414.336.75.75.75h15.75a.75.75 0 000-1.5H5.378A2.25 2.25 0 017.5 15h11.218a.75.75 0 00.674-.421 60.358 60.358 0 002.96-7.228.75.75 0 00-.525-.965A60.864 60.864 0 005.68 4.509l-.232-.867A1.875 1.875 0 003.636 2.25H2.25zM3.75 20.25a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM16.5 20.25a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0z" />
            </svg>
          </Navbar.Link>
          <Navbar.Link
            style={{ fontSize: "22px" }}
            href="/shop"
          >
            LOGIN
          </Navbar.Link>
          <Navbar.Link
            style={{ fontSize: "22px" }}
            href="/errors"
          >
            SIGN-UP
          </Navbar.Link>
        </Navbar.Collapse>
      </Navbar>
      <div className="py-60">{children}</div>
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
