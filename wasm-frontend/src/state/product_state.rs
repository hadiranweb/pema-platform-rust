use models::product::Product;
use models::pagination::PaginatedResponse;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ProductState {
    pub products: Vec<Product>,
    pub current_product: Option<Product>,
    pub pagination: Option<PaginationInfo>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub search_term: String,
    pub selected_category: Option<String>,
    pub filters: ProductFilters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductFilters {
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub in_stock: Option<bool>,
    pub vendor_id: Option<Uuid>,
}

impl Default for ProductState {
    fn default() -> Self {
        Self {
            products: Vec::new(),
            current_product: None,
            pagination: None,
            is_loading: false,
            error: None,
            search_term: String::new(),
            selected_category: None,
            filters: ProductFilters::default(),
        }
    }
}

impl Default for ProductFilters {
    fn default() -> Self {
        Self {
            min_price: None,
            max_price: None,
            in_stock: None,
            vendor_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProductAction {
    LoadProductsStart,
    LoadProductsSuccess(PaginatedResponse<Product>),
    LoadProductsFailure(String),
    LoadProductStart(Uuid),
    LoadProductSuccess(Product),
    LoadProductFailure(String),
    CreateProductStart,
    CreateProductSuccess(Product),
    CreateProductFailure(String),
    UpdateProductStart(Uuid),
    UpdateProductSuccess(Product),
    UpdateProductFailure(String),
    DeleteProductStart(Uuid),
    DeleteProductSuccess(Uuid),
    DeleteProductFailure(String),
    SetSearchTerm(String),
    SetCategory(Option<String>),
    SetFilters(ProductFilters),
    SetCurrentProduct(Option<Product>),
    ClearError,
}

impl ProductState {
    pub fn reduce(self, action: ProductAction) -> Self {
        match action {
            ProductAction::LoadProductsStart => Self {
                is_loading: true,
                error: None,
                ..self
            },
            ProductAction::LoadProductsSuccess(paginated_response) => Self {
                products: paginated_response.items,
                pagination: Some(PaginationInfo {
                    current_page: paginated_response.current_page,
                    total_pages: paginated_response.total_pages,
                    total_items: paginated_response.total_items,
                    limit: paginated_response.limit,
                }),
                is_loading: false,
                error: None,
                ..self
            },
            ProductAction::LoadProductsFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            ProductAction::LoadProductStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            ProductAction::LoadProductSuccess(product) => Self {
                current_product: Some(product),
                is_loading: false,
                error: None,
                ..self
            },
            ProductAction::LoadProductFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            ProductAction::CreateProductStart => Self {
                is_loading: true,
                error: None,
                ..self
            },
            ProductAction::CreateProductSuccess(product) => {
                let mut products = self.products.clone();
                products.insert(0, product);
                Self {
                    products,
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            ProductAction::CreateProductFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            ProductAction::UpdateProductStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            ProductAction::UpdateProductSuccess(updated_product) => {
                let products = self.products.into_iter()
                    .map(|p| if p.id == updated_product.id { updated_product.clone() } else { p })
                    .collect();
                Self {
                    products,
                    current_product: Some(updated_product),
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            ProductAction::UpdateProductFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            ProductAction::DeleteProductStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            ProductAction::DeleteProductSuccess(product_id) => {
                let products = self.products.into_iter()
                    .filter(|p| p.id != product_id)
                    .collect();
                let current_product = if self.current_product.as_ref().map(|p| p.id) == Some(product_id) {
                    None
                } else {
                    self.current_product
                };
                Self {
                    products,
                    current_product,
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            ProductAction::DeleteProductFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            ProductAction::SetSearchTerm(search_term) => Self {
                search_term,
                ..self
            },
            ProductAction::SetCategory(category) => Self {
                selected_category: category,
                ..self
            },
            ProductAction::SetFilters(filters) => Self {
                filters,
                ..self
            },
            ProductAction::SetCurrentProduct(product) => Self {
                current_product: product,
                ..self
            },
            ProductAction::ClearError => Self {
                error: None,
                ..self
            },
        }
    }
}
