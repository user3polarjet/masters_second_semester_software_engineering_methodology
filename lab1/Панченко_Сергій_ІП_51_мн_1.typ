#set text(font: "Times New Roman", size: 14pt)
#set page(
  paper: "a4",
  margin: (top: 1.5cm, bottom: 1.5cm, left: 2.5cm, right: 1.5cm)
)
#align(center)[
#image("kpi.png", width: 75%)

Міністерство освіти і науки України

Національний технічний університет України

“Київський політехнічний інститут імені Ігоря Сікорського”

Факультет інформатики та обчислювальної техніки

Кафедра інформатики та програмної інженерії

#align(horizon)[
  #text(size: 18pt)[
    *Лабораторна робота №1*

    Методологія інженерії програмного забезпечення
    ]

    Тема: Польові дослідження
  ]

  #columns(2, gutter: 8pt)[
    #align(left)[
      Виконав:

      студент групи ІП-51мн

      Панченко С. В.
    ]

    #colbreak()

    #align(right)[
      Перевірив:

      Щебланін Ю. М.
    ]
  ]
]

#align(center + bottom)[
  Київ 2026
]

#set page(numbering: "1")
#show outline: it => {
  show heading: set align(center)
  show heading: set text(weight: "regular")
  it
}
#outline(title: upper([Зміст]))

#set heading(numbering: (..nums) => nums.pos().map(str).join("."))
#show heading: it => {
  if it.level == 1 {
    counter(figure.where(kind: image)).update(0)
    set align(center)
    set text(weight: "regular", size: 18pt)
    pagebreak()
    upper(it)
  } else {
    set text(weight: "regular", size: 14pt)
    it
  }
}
#show figure: it => {
  set align(center)
  it.body
  v(8pt, weak: true)
  it.supplement 
  [ ]
  context (it.counter.display(it.numbering))
  [ — ] 
  it.caption.body
}
#set figure(
  supplement: [Рисунок],
  numbering: (num) => {
    context {
      let h_num = counter(heading).at(here()).at(0)
      str(h_num) + "." + str(num)
    }
  }
)

#show ref: it => {
  let el = it.element
  if el != none and el.func() == figure {
    context {
      let num = el.counter.at(el.location())
      numbering(el.numbering, ..num)
    }
  } else {
    it
  }
}
#set par(first-line-indent: (amount: 1.25cm, all: true), justify: true, leading: 1em, spacing: 1em)
#set list(indent: 1.25cm)
#set enum(indent: 1.25cm)

= Мета
Ознайомитися з типами польових досліджень (Field studies, Field experiments).

= Завдання

+ Визначити мету дослідження та обрати тип (варіант) та метод дослідження. Погодити з викладачем.
+ Створити план дослідження та обрати або створити необхідні інструменти дослідження.
+ Виконати дослідження.
+ Аналізувати дані дослідження та зробити висновки.
+ Створити звіт (протокол та презентацію) за результатами дослідження за обраним варіантом завдання дослідження.

= Виконання

Об'єктом дослідження було обрано репозиторій відкритого коду `typst/typst` на платформі GitHub. Для виконання завдання застосовано тип дослідження "Польові дослідження" (Field Studies), зокрема методики третього ступеня (Third Degree): *Analysis of Electronic Databases of Work Performed* (аналіз метаданих понад 2000 Pull Requests) та *Documentation Analysis* (аналіз текстових описів артефактів).

== Перевірка «Закону тривіальності»

На рисунках @1_size_vs_density_unfiltered та @2_size_vs_density_filtered зображено залежність між обсягом внесених змін (кількість доданих та видалених рядків) та щільністю обговорення (кількість коментарів на рядок коду). 

#figure(
  image("build/plots/1_size_vs_density_unfiltered.svg", width: 90%),
  caption: [Залежність розміру PR від щільності коментарів (всі дані)]
) <1_size_vs_density_unfiltered>

#figure(
  image("build/plots/2_size_vs_density_filtered.svg", width: 90%),
  caption: [Залежність розміру PR від щільності коментарів (до 1000 рядків)]
) <2_size_vs_density_filtered>

Діаграма розсіювання демонструє класичну L-подібну залежність: дрібні PR (до 100 рядків) викликають непропорційно більшу кількість коментарів, тоді як великі рефакторинги (>1000 рядків) проходять рев'ю майже без обговорень. 

Крім того, рисунок @3_files_vs_comments демонструє відсутність лінійної масштабованості між кількістю змінених файлів та кількістю коментарів. PR, що зачіпають понад 40 файлів, мають вкрай низьку кількість коментарів, що вказує на виникнення «когнітивної стелі» у рев'юерів.

#figure(
  image("build/plots/3_files_vs_comments.svg", width: 90%),
  caption: [Кореляція кількості змінених файлів та коментарів]
) <3_files_vs_comments>

== Аналіз документації як «комунікаційного запобіжника»

Метою наступного етапу було дослідити вплив якості текстового опису Pull Request на комунікаційне навантаження. Результати наведено на рисунках @4_body_len_vs_comment_count та @5_body_len_vs_total_comment_len.

#figure(
  image("build/plots/4_body_len_vs_comment_count.svg", width: 90%),
  caption: [Залежність кількості коментарів від довжини опису PR]
) <4_body_len_vs_comment_count>

#figure(
  image("build/plots/5_body_len_vs_total_comment_len.svg", width: 90%),
  caption: [Залежність загального обсягу дискусії від довжини опису PR]
) <5_body_len_vs_total_comment_len>

Спостерігається клиноподібний розподіл: PR із мінімальним описом (менше 500 символів) генерують найбільші комунікаційні сплески (аномалії до 70 000 символів у коментарях). Детальний опис локалізує обговорення, роблячи рев'ю більш передбачуваним.

== Динаміка Code Review: Синдром «Пінг-Понгу»

На рисунку @6_ping_pong_syndrome проаналізовано вплив мікро-ітерацій (чергування коментаря рев'юера та дрібного коміту автора) на загальний час життєвого циклу Пул-реквесту.

#figure(
  image("build/plots/6_ping_pong_syndrome.svg"),
  caption: [Синдром Пінг-Понгу: вплив циклів чергування на час злиття]
) <6_ping_pong_syndrome>

Зібрана статистика виявляє майже експоненціальне зростання часу інтеграції: у базовому сценарії (0 циклів) процес займає в середньому 1.7 днів. При наявності 6 і більше циклів комунікаційного тертя розробка паралізується, і час до злиття сягає понад 30 днів.

= Висновок

У ході польового дослідження екосистеми typst/typst доведено, що швидкість розробки суттєво залежить від соціально-комунікаційних патернів. Використання методів Analysis of Electronic Databases та Documentation Analysis дозволило підтвердити закон тривіальності: спільнота схильна до мікроменеджменту дрібних правок, уникаючи глибокого аналізу масштабних змін через когнітивне перевантаження.

Дослідження також підтвердило критичну важливість якісної документації артефактів. Відсутність початкового контексту в PR провокує тривалі дискусії, тоді як детальний опис знижує їхній обсяг у 3-5 разів. Водночас виявлено деструктивний вплив синдрому «Пінг-Понгу»: ітеративний формат спілкування під час Code Review є вкрай неефективним і здатен збільшити час прийняття рішень до 17 разів.

Для оптимізації процесів розробки команді рекомендується впровадити обов'язкові шаблони описів коду та перейти до пакетного рецензування, що дозволить мінімізувати накладні витрати на постійну зміну контексту.

#counter(heading).update(0)
#let ukr_alphabet = ("А", "Б", "В", "Г", "Д", "Е", "Є", "Ж", "З", "И", "І", "Ї", "Й", "К", "Л", "М", "Н", "О", "П", "Р", "С", "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Щ", "Ю", "Я")
  
#set heading(numbering: (..nums) => {
  let values = nums.pos()
  let n = values.at(0)
  
  let letter = if n <= ukr_alphabet.len() {
    ukr_alphabet.at(n - 1)
  } else {
    str(n)
  }
  
  if values.len() == 1 {
    letter
  } else {
    letter + "." + values.slice(1).map(str).join(".")
  }
})

#show heading: it => {
  if it.level == 1 {
    counter(figure.where(kind: image)).update(0)
    
    set align(center)
    set text(weight: "regular", size: 18pt)
    pagebreak()
    
    block[
      Додаток #counter(heading).display() \
      #text(it.body)
    ]
  } else {
    set text(weight: "regular", size: 14pt)
    it
  }
}

= Лістинг коду

#let embed_python(file_path) = {
  heading(file_path, level: 2)
  raw(read(file_path), lang: "python", block: true)
}

#embed_python("main.py")
#embed_python("fetch_prs.py")
#embed_python("pr_classes.py")