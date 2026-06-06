use rusqlite::Connection;

pub fn seed_data(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let admin_hash = bcrypt::hash("admin123", 4)?;
    conn.execute(
        "INSERT INTO users (username, password_hash, display_name, role) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["admin", admin_hash, "系统管理员", "admin"],
    )?;

    let staff_hash = bcrypt::hash("staff123", 4)?;
    conn.execute(
        "INSERT INTO users (username, password_hash, display_name, role) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["zhangsan", staff_hash, "张三", "staff"],
    )?;

    let staff_hash2 = bcrypt::hash("staff123", 4)?;
    conn.execute(
        "INSERT INTO users (username, password_hash, display_name, role) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["lisi", staff_hash2, "李四", "staff"],
    )?;

    let artifacts = [
        ("青花缠枝莲纹瓶", "陶瓷", "明代", "瓷器", "高45cm 口径12cm", "景德镇窑", "明代青花瓷精品，缠枝莲纹饰精美，釉色温润", "在库"),
        ("商代后母戊鼎", "青铜器", "商代", "青铜", "高133cm 口径110cm", "河南安阳", "商代晚期青铜重器，国之重宝", "在展"),
        ("唐三彩骆驼载乐俑", "陶瓷", "唐代", "陶器", "高58cm", "陕西西安", "唐代三彩精品，骆驼上载有乐舞人物", "在库"),
        ("宋代汝窑天青釉碗", "陶瓷", "宋代", "瓷器", "高7cm 口径17cm", "河南宝丰", "汝窑传世精品，天青色釉面温润如玉", "养护中"),
        ("明代宣德炉", "青铜器", "明代", "铜器", "高12cm", "北京", "宣德年间铸造的铜香炉，工艺精湛", "在库"),
        ("清代翡翠白菜", "玉器", "清代", "翡翠", "高18cm", "清宫旧藏", "以天然翡翠雕琢而成，菜叶上栖息螽斯与蝗虫", "在展"),
        ("战国编钟", "青铜器", "战国", "青铜", "高68cm", "湖北随州", "战国时期青铜乐器，音律精准", "修复中"),
        ("宋代缂丝花鸟图", "丝织品", "宋代", "丝织", "纵120cm 横60cm", "苏州", "宋代缂丝工艺精品，花鸟图案栩栩如生", "在库"),
        ("西周大克鼎", "青铜器", "西周", "青铜", "高93cm", "陕西扶风", "西周孝王时期青铜重器，铭文290字", "在库"),
        ("唐代仕女图卷", "书画", "唐代", "绢本设色", "纵46cm 横180cm", "陕西西安", "唐代仕女画精品，描绘宫廷贵妇生活场景", "养护中"),
    ];

    for (name, category, era, material, dimensions, origin, description, status) in &artifacts {
        conn.execute(
            "INSERT INTO artifacts (name, category, era, material, dimensions, origin, description, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![name, category, era, material, dimensions, origin, description, status],
        )?;
    }

    let conservations = [
        (4, "温湿度调控养护", "李明", "2024-01-15", "2024-03-20", "对汝窑天青釉碗进行温湿度调控养护，防止釉面开裂"),
        (10, "书画修复性养护", "王芳", "2024-02-01", "2024-04-15", "对唐代仕女图卷进行修复性养护，加固绢本纤维"),
        (1, "定期检查养护", "李明", "2024-03-01", "2024-03-15", "对青花缠枝莲纹瓶进行定期检查养护，状态良好"),
    ];

    for (artifact_id, method, performer, start_date, end_date, notes) in &conservations {
        conn.execute(
            "INSERT INTO conservations (artifact_id, method, performer, start_date, end_date, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![artifact_id, method, performer, start_date, end_date, notes],
        )?;
    }

    let exhibitions = [
        ("华夏青铜文明展", "国家博物馆一号展厅", "2024-06-01", "2024-12-31", "张馆长", "展示中华青铜文明的辉煌成就，汇集商周至汉代青铜精品", "进行中"),
        ("丝路遗珍——唐代文物特展", "省博物馆特展厅", "2024-09-01", "2025-03-01", "李馆长", "丝绸之路沿线出土的唐代珍贵文物特展", "进行中"),
        ("宋韵风华——宋代瓷器精品展", "市博物馆三楼展厅", "2025-01-15", "2025-06-30", "王馆长", "汇集宋代五大名窑精品，展现宋代瓷器之美", "筹备中"),
    ];

    for (name, venue, start_date, end_date, curator, description, status) in &exhibitions {
        conn.execute(
            "INSERT INTO exhibitions (name, venue, start_date, end_date, curator, description, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![name, venue, start_date, end_date, curator, description, status],
        )?;
    }

    conn.execute("INSERT INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (1, 2)", [])?;
    conn.execute("INSERT INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (1, 7)", [])?;
    conn.execute("INSERT INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (1, 9)", [])?;
    conn.execute("INSERT INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (2, 3)", [])?;
    conn.execute("INSERT INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (2, 6)", [])?;
    conn.execute("INSERT INTO exhibition_artifacts (exhibition_id, artifact_id) VALUES (2, 10)", [])?;

    let restorations = [
        (7, "赵师傅", "青铜器除锈与补配修复", "2024-02-01", "", 15000.0, "对战国编钟进行除锈处理和缺失部位补配修复", "进行中"),
        (5, "钱师傅", "铜器抛光与保护层修复", "2023-11-01", "2024-01-20", 8000.0, "对宣德炉进行表面抛光处理并涂覆保护层", "已完成"),
    ];

    for (artifact_id, restorer, method, start_date, end_date, cost, description, status) in &restorations {
        conn.execute(
            "INSERT INTO restorations (artifact_id, restorer, method, start_date, end_date, cost, description, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![artifact_id, restorer, method, start_date, end_date, cost, description, status],
        )?;
    }

    let reservations = [
        ("刘先生", "13800138001", "2024-07-15", 2, Some(1), "已确认"),
        ("陈女士", "13900139002", "2024-08-20", 4, Some(2), "已确认"),
        ("王先生", "13700137003", "2024-09-10", 3, Some(1), "待确认"),
        ("赵女士", "13600136004", "2025-02-01", 5, Some(3), "待确认"),
        ("孙先生", "13500135005", "2024-10-05", 2, Some(2), "已取消"),
    ];

    for (visitor_name, phone, visit_date, visitor_count, exhibition_id, status) in &reservations {
        conn.execute(
            "INSERT INTO reservations (visitor_name, phone, visit_date, visitor_count, exhibition_id, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![visitor_name, phone, visit_date, visitor_count, exhibition_id, status],
        )?;
    }

    Ok(())
}
