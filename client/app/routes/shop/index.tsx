import { Label, Select } from "flowbite-react";

export default function Shop() {
  return (
    <div className="container mt-5">
      <div className="row">
        <section className="col-3">
          <div id="select">
            <div className="mb-2 block">
              <Label
                htmlFor="countries"
                value="SORT"
              />
            </div>
            <Select
              id="countries"
              required={true}
            >
              <option>United States</option>
              <option>Canada</option>
              <option>France</option>
              <option>Germany</option>
            </Select>
          </div>
        </section>
      </div>
    </div>
  );
}
