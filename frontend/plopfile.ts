import { NodePlopAPI } from "plop";
export default function (plop: NodePlopAPI) {
  plop.setGenerator("feature", {
    description: "Feature skeleton",
    prompts: [
      {
        type: "input",
        name: "feature-name",
        message: "Enter feature name",
      },
      {
        type: "input",
        name: "route-path",
        message: "Enter page route",
      },
    ],
    actions: [
      {
        type: "add",
        path: "./src/features/{{feature-name}}/index.ts",
        templateFile: "./plop-template/index.hbs",
      },
      {
        type: "add",
        path: "./src/features/{{feature-name}}/components/{{feature-name}}.tsx",
        templateFile: "./plop-template/main-component.hbs",
      },
      {
        type: "add",
        path: "./src/features/{{feature-name}}/routes.tsx",
        templateFile: "./plop-template/routes.hbs",
      },
      {
        type: "add",
        path: "./src/features/{{feature-name}}/hooks/{{feature-name}}.ts",
      },
      {
        type: "add",
        path: "./src/features/{{feature-name}}/services/{{feature-name}}.api.ts",
      },
      {
        type: "add",
        path: "./src/features/{{feature-name}}/types/index.ts",
      },
      {
        type: "append",
        path: "./src/commons/routes/index.tsx",
        // pattern: /(import.*from.*@\/features\/.*;)/,
        pattern: /(import { [A-Z_]+_ROUTE_PATH } from "@\/features\/[^"]+";)/,
        template:
          'import { {{upperCase (snakeCase feature-name)}}_ROUTE_PATH } from "@/features/{{feature-name}}";',
      },
      {
        type: "append",
        path: "./src/commons/routes/index.tsx",
        pattern: /(<Link[\s\S]*<\/Link>)/,
        templateFile: "./plop-template/route-link.hbs",
      },
      {
        type: "append",
        path: "./src/_root.ts",
        pattern: 'import { rootRoute } from "@/commons/routes";',
        template:
          'import { {{camelCase feature-name}}Route } from "@/features/{{feature-name}}";',
      },
      {
        type: "append",
        path: "./src/_root.ts",
        pattern: "const routeTree = rootRoute.addChildren([",
        template: "{{camelCase feature-name}}Route,",
      },
    ],
  });
}
