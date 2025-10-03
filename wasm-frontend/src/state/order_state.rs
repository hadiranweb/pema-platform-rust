use models::order::Order;
use models::pagination::PaginatedResponse;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct OrderState {
    pub orders: Vec<Order>,
    pub current_order: Option<Order>,
    pub pagination: Option<PaginationInfo>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub status_filter: Option<String>,
    pub user_filter: Option<Uuid>,
    pub date_range: Option<DateRange>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateRange {
    pub from: chrono::DateTime<chrono::Utc>,
    pub to: chrono::DateTime<chrono::Utc>,
}

impl Default for OrderState {
    fn default() -> Self {
        Self {
            orders: Vec::new(),
            current_order: None,
            pagination: None,
            is_loading: false,
            error: None,
            status_filter: None,
            user_filter: None,
            date_range: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OrderAction {
    LoadOrdersStart,
    LoadOrdersSuccess(PaginatedResponse<Order>),
    LoadOrdersFailure(String),
    LoadOrderStart(Uuid),
    LoadOrderSuccess(Order),
    LoadOrderFailure(String),
    CreateOrderStart,
    CreateOrderSuccess(Order),
    CreateOrderFailure(String),
    UpdateOrderStart(Uuid),
    UpdateOrderSuccess(Order),
    UpdateOrderFailure(String),
    UpdateOrderStatusStart(Uuid),
    UpdateOrderStatusSuccess(Order),
    UpdateOrderStatusFailure(String),
    CancelOrderStart(Uuid),
    CancelOrderSuccess(Order),
    CancelOrderFailure(String),
    SetStatusFilter(Option<String>),
    SetUserFilter(Option<Uuid>),
    SetDateRange(Option<DateRange>),
    SetCurrentOrder(Option<Order>),
    ClearError,
}

impl OrderState {
    pub fn reduce(self, action: OrderAction) -> Self {
        match action {
            OrderAction::LoadOrdersStart => Self {
                is_loading: true,
                error: None,
                ..self
            },
            OrderAction::LoadOrdersSuccess(paginated_response) => Self {
                orders: paginated_response.items,
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
            OrderAction::LoadOrdersFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            OrderAction::LoadOrderStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            OrderAction::LoadOrderSuccess(order) => Self {
                current_order: Some(order),
                is_loading: false,
                error: None,
                ..self
            },
            OrderAction::LoadOrderFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            OrderAction::CreateOrderStart => Self {
                is_loading: true,
                error: None,
                ..self
            },
            OrderAction::CreateOrderSuccess(order) => {
                let mut orders = self.orders.clone();
                orders.insert(0, order);
                Self {
                    orders,
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            OrderAction::CreateOrderFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            OrderAction::UpdateOrderStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            OrderAction::UpdateOrderSuccess(updated_order) => {
                let orders = self.orders.into_iter()
                    .map(|o| if o.id == updated_order.id { updated_order.clone() } else { o })
                    .collect();
                Self {
                    orders,
                    current_order: Some(updated_order),
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            OrderAction::UpdateOrderFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            OrderAction::UpdateOrderStatusStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            OrderAction::UpdateOrderStatusSuccess(updated_order) => {
                let orders = self.orders.into_iter()
                    .map(|o| if o.id == updated_order.id { updated_order.clone() } else { o })
                    .collect();
                Self {
                    orders,
                    current_order: Some(updated_order),
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            OrderAction::UpdateOrderStatusFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            OrderAction::CancelOrderStart(_) => Self {
                is_loading: true,
                error: None,
                ..self
            },
            OrderAction::CancelOrderSuccess(cancelled_order) => {
                let orders = self.orders.into_iter()
                    .map(|o| if o.id == cancelled_order.id { cancelled_order.clone() } else { o })
                    .collect();
                Self {
                    orders,
                    current_order: Some(cancelled_order),
                    is_loading: false,
                    error: None,
                    ..self
                }
            },
            OrderAction::CancelOrderFailure(error) => Self {
                is_loading: false,
                error: Some(error),
                ..self
            },
            OrderAction::SetStatusFilter(status_filter) => Self {
                status_filter,
                ..self
            },
            OrderAction::SetUserFilter(user_filter) => Self {
                user_filter,
                ..self
            },
            OrderAction::SetDateRange(date_range) => Self {
                date_range,
                ..self
            },
            OrderAction::SetCurrentOrder(order) => Self {
                current_order: order,
                ..self
            },
            OrderAction::ClearError => Self {
                error: None,
                ..self
            },
        }
    }
}
