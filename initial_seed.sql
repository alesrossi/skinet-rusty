insert into product_brands (id, name)
values
    (1, 'Angular'),
    (2, 'NetCore'),
    (3, 'VS Code'),
    (4, 'React'),
    (5, 'Typescript'),
    (6, 'Redis')
;

insert into product_types (id, name)
values
    (1, 'Boards'),
    (2, 'Hats'),
    (3, 'Boots'),
    (4, 'Gloves')
;

insert into products (id, name, cost, description, pictureurl, producttype, productbrand)
values (
           DEFAULT,
           'Angular Speedster Board 2000',
           200,
           'Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas porttitor congue massa. Fusce posuere, magna sed pulvinar ultricies, purus lectus malesuada libero, sit amet commodo magna eros quis urna.',
           'images/products/sb-ang1.png',
           1,
           1
       ),
       (
           DEFAULT,
           'Green Angular Board 3000',
           150,
           'Nunc viverra imperdiet enim. Fusce est. Vivamus a tellus.',
           'images/products/sb-ang2.png',
           1,
           1
       ),
       (
           DEFAULT,
           'Core Board Speed Rush 3',
           180,
           'Suspendisse dui purus, scelerisque at, vulputate vitae, pretium mattis, nunc. Mauris eget neque at sem venenatis eleifend. Ut nonummy.',
           'images/products/sb-core1.png',
           1,
           2
       ),
       (
           DEFAULT,
           'Net Core Super Board',
           300,
           'Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin pharetra nonummy pede. Mauris et orci.',
           'images/products/sb-core2.png',
           1,
           2
       ),
       (
           DEFAULT,
           'React Board Super Whizzy Fast',
           250,
           'Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas porttitor congue massa. Fusce posuere, magna sed pulvinar ultricies, purus lectus malesuada libero, sit amet commodo magna eros quis urna.',
           'images/products/sb-react1.png',
           1,
           4
       ),
       (
           DEFAULT,
           'Typescript Entry Board',
           120,
           'Aenean nec lorem. In porttitor. Donec laoreet nonummy augue.',
           'images/products/sb-ts1.png',
           1,
           5
       ),
       (
           DEFAULT,
           'Core Blue Hat',
           10,
           'Fusce posuere, magna sed pulvinar ultricies, purus lectus malesuada libero, sit amet commodo magna eros quis urna.',
           'images/products/hat-core1.png',
           2,
           2
       ),
       (
           DEFAULT,
           'Green React Woolen Hat',
           8,
           'Suspendisse dui purus, scelerisque at, vulputate vitae, pretium mattis, nunc. Mauris eget neque at sem venenatis eleifend. Ut nonummy.',
           'images/products/hat-react1.png',
           2,
           4
       ),
       (
           DEFAULT,
           'Purple React Woolen Hat',
           15,
           'Fusce posuere, magna sed pulvinar ultricies, purus lectus malesuada libero, sit amet commodo magna eros quis urna.',
           'images/products/hat-react2.png',
           2,
           4
       ),
       (
           DEFAULT,
           'Blue Code Gloves',
           18,
           'Nunc viverra imperdiet enim. Fusce est. Vivamus a tellus.',
           'images/products/glove-code1.png',
           4,
           3
       ),
       (
           DEFAULT,
           'Green Code Gloves',
           15,
           'Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin pharetra nonummy pede. Mauris et orci.',
           'images/products/glove-code2.png',
           4,
           3
       ),
       (
           DEFAULT,
           'Purple React Gloves',
           16,
           'Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas porttitor congue massa.',
           'images/products/glove-react1.png',
           4,
           4
       ),
       (
           DEFAULT,
           'Green React Gloves',
           14,
           'Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin pharetra nonummy pede. Mauris et orci.',
           'images/products/glove-react2.png',
           4,
           4
       ),
       (
           DEFAULT,
           'Redis Red Boots',
           250,
           'Suspendisse dui purus, scelerisque at, vulputate vitae, pretium mattis, nunc. Mauris eget neque at sem venenatis eleifend. Ut nonummy.',
           'images/products/boot-redis1.png',
           3,
           6
       ),
       (
           DEFAULT,
           'Core Red Boots',
           189.99,
           'Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas porttitor congue massa. Fusce posuere, magna sed pulvinar ultricies, purus lectus malesuada libero, sit amet commodo magna eros quis urna.',
           'images/products/boot-core2.png',
           3,
           2
       ),
       (
           DEFAULT,
           'Core Purple Boots',
           199.99,
           'Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin pharetra nonummy pede. Mauris et orci.',
           'images/products/boot-core1.png',
           3,
           2
       ),
       (
           DEFAULT,
           'Angular Purple Boots',
           150,
           'Aenean nec lorem. In porttitor. Donec laoreet nonummy augue.',
           'images/products/boot-ang2.png',
           3,
           1
       ),
       (
           DEFAULT,
           'Angular Purple Boots',
           180,
           'Suspendisse dui purus, scelerisque at, vulputate vitae, pretium mattis, nunc. Mauris eget neque at sem venenatis eleifend. Ut nonummy.',
           'images/products/boot-ang1.png',
           3,
           1
       )
;