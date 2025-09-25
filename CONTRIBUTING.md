# Contributing to Ngrok Desktop App

Thank you for your interest in contributing to Ngrok Desktop App! We welcome contributions from the community and are pleased to have you join us.

## 🚀 Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/ngrok-app.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Commit your changes: `git commit -m 'Add some amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## 📋 Development Setup

### Prerequisites

- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
- **npm** or **pnpm** (we recommend pnpm)

### Installation

```bash
# Clone the repository
git clone https://github.com/Filipponik/ngrok-app.git
cd ngrok-app

# Install dependencies
pnpm install

# Start development server
pnpm run tauri dev
```

## 🏗️ Project Structure

```
ngrok-app/
├── src/                    # Vue.js frontend
│   ├── components/         # Reusable Vue components
│   │   ├── HeadersEditor.vue    # Custom headers management
│   │   ├── CreateTunnel.vue     # Tunnel creation form
│   │   └── ...
│   ├── pages/             # Application pages/routes
│   ├── services/          # API services and business logic
│   ├── router/            # Vue Router configuration
│   └── assets/            # Static assets (CSS, images)
├── src-tauri/             # Rust backend (Tauri)
│   ├── src/               # Rust source code
│   ├── Cargo.toml         # Rust dependencies
│   └── icons/             # Application icons
└── public/                # Public assets
```

## 💻 Development Guidelines

### Code Style

**Frontend (Vue/TypeScript)**
- Use **TypeScript** for all new code
- Follow **Vue 3 Composition API** patterns
- Use **`<script setup>`** syntax
- Follow **Element Plus** design system
- Use **Tailwind CSS** for styling

**Backend (Rust)**
- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` for linting
- Write meaningful error messages
- Use proper error handling with `Result<T, E>`

### Component Guidelines

1. **Props**: Always define TypeScript interfaces for props
2. **Emits**: Use typed emits with proper event definitions
3. **Reactivity**: Prefer `ref()` and `computed()` from Composition API
4. **Naming**: Use PascalCase for components, camelCase for functions

### Example Component Structure

```vue
<script setup lang="ts">
import { ref, computed } from "vue";

// Define interfaces
interface Props {
  modelValue: string[];
  maxItems?: number;
}

// Define props with defaults
const props = withDefaults(defineProps<Props>(), {
  maxItems: 10
});

// Define emits
const emit = defineEmits<{
  "update:modelValue": [value: string[]];
}>();

// Component logic
const items = ref<string[]>([...props.modelValue]);
</script>

<template>
  <!-- Template with proper accessibility -->
  <div class="component-wrapper">
    <!-- Content -->
  </div>
</template>

<style scoped>
/* Component-specific styles */
</style>
```

## 🧪 Testing

### Frontend Testing
```bash
# Run type checking
npm run build

# Check for linting issues
npm run lint  # (when available)
```

### Backend Testing
```bash
# Check Rust code
cd src-tauri
cargo check
cargo clippy
cargo fmt --check
```

## 📝 Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

**Examples:**
```
feat: add support for custom response headers
fix: resolve tunnel creation timeout issue
docs: update installation instructions
style: format code with prettier
refactor: extract header validation logic
```

## 🐛 Bug Reports

When filing a bug report, please include:

1. **Operating System** (Windows, macOS, Linux + version)
2. **App Version**
3. **Steps to reproduce** the issue
4. **Expected behavior**
5. **Actual behavior**
6. **Screenshots** (if applicable)
7. **Console logs** (if available)

## 💡 Feature Requests

For feature requests, please provide:

1. **Clear description** of the feature
2. **Use case** - why is this needed?
3. **Proposed solution** (if you have ideas)
4. **Alternative solutions** considered
5. **Additional context** (mockups, examples, etc.)

## 🔍 Code Review Process

1. **Automated checks** must pass (TypeScript compilation, Rust compilation)
2. **Code review** by at least one maintainer
3. **Testing** on different platforms (when applicable)
4. **Documentation** updates (if needed)

### What We Look For

- ✅ **Code Quality**: Clean, readable, well-structured
- ✅ **Performance**: No unnecessary re-renders or heavy operations
- ✅ **Accessibility**: Proper ARIA labels, keyboard navigation
- ✅ **Consistency**: Follows existing patterns and styles
- ✅ **Documentation**: Code comments where necessary

## 🎨 UI/UX Guidelines

- Follow **Element Plus** design system
- Maintain **consistent spacing** using Tailwind utilities
- Ensure **responsive design** works on all screen sizes
- Use **semantic HTML** elements
- Test **keyboard navigation**
- Maintain **color contrast** ratios for accessibility

## 🚀 Release Process

1. Version bump in `package.json` and `src-tauri/Cargo.toml`
2. Update `CHANGELOG.md` with new features and fixes
3. Create release tag
4. Build for all platforms
5. Publish release with artifacts

## ❓ Questions?

- **General questions**: Open a [Discussion](https://github.com/Filipponik/ngrok-app/discussions)
- **Bug reports**: Create an [Issue](https://github.com/Filipponik/ngrok-app/issues)
- **Feature requests**: Create an [Issue](https://github.com/Filipponik/ngrok-app/issues) with the `enhancement` label

## 📜 Code of Conduct

We are committed to fostering a welcoming community. Please be respectful and constructive in all interactions.

- **Be respectful** of differing viewpoints and experiences
- **Use welcoming and inclusive language**
- **Focus on what is best** for the community
- **Show empathy** towards other community members

---

**Thank you for contributing to Ngrok Desktop App! 🎉**

Every contribution, no matter how small, makes this project better for everyone.
